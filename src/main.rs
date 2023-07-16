use anyhow::Context;
use hyprland::data::{Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListener;
use hyprland::prelude::*;
use hyprland::shared::Address;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;
use std::thread;

/// Report the current state of the workspace list.
///
/// Whenever the workspace list may have changed, this is called.
///
/// We _could_ be smart here and figure out exactly what changed since last time and update our
/// state rather than re-query it each time, but _shrug_. This is easy.
fn report() -> anyhow::Result<()> {
    // We can't get empty workspaces: https://github.com/hyprwm/hyprland-wiki/issues/269
    let wses = Workspaces::get().context("get all workspaces")?;
    let mut ws_by_monitor = wses.fold(HashMap::new(), |mut hm, ws| {
        hm.entry(ws.monitor.clone())
            .or_insert_with(Vec::new)
            .push(ws);
        hm
    });

    // Sort workspaces by id
    for (_, wses) in &mut ws_by_monitor {
        wses.sort_by_key(|ws| ws.id);

        // Also inject any missing (presumably empty) workspaces to avoid jumping
        let mx = wses.last().unwrap().id;
        let mut old_wses =
            VecDeque::from(std::mem::replace(wses, Vec::with_capacity(mx as usize + 1)));
        for i in 1..=mx {
            if i == old_wses[0].id {
                wses.push(old_wses.pop_front().unwrap());
            } else {
                wses.push(Workspace {
                    id: i,
                    name: format!("{i}"),
                    monitor: old_wses[0].monitor.clone(),
                    windows: 0,
                    fullscreen: false,
                    last_window: Address::new(""),
                    last_window_title: String::new(),
                });
            }
        }
    }

    // Sort monitors left-to-right, top-to-bottom:
    let mut monitors = Monitors::get().context("get monitors")?.to_vec();
    monitors.sort_by_key(|m| {
        (
            (m.x + i32::from(m.width)) / 2,
            (m.y + i32::from(m.height)) / 2,
        )
    });

    let mut i = 1;
    for m in &monitors {
        let Some(wses) = ws_by_monitor.get(&m.name) else {
            eprintln!("no workspaces on monitor {}", m.name);
            continue;
        };
        for (wsi, ws) in wses.into_iter().enumerate() {
            println!("workspace_{i}|string|{}", ws.name);
            println!("workspace_{i}_windows|int|{}", ws.windows);
            println!("workspace_{i}_index_on_monitor|int|{wsi}");
            println!("workspace_{i}_monitor|string|{}", m.name);
            println!(
                "workspace_{i}_active|bool|{:?}",
                m.active_workspace.id == ws.id
            );
            println!(
                "workspace_{i}_focused|bool|{:?}",
                m.active_workspace.id == ws.id && m.focused
            );
            i += 1;
        }
    }
    // And to help with rendering since yambar is unhelpful
    println!("workspace_count|int|{i}");
    println!("");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Allow multiple different events to trigger a re-report.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for _ in rx {
            if let Err(e) = report() {
                eprintln!("{:?}", e);
                return;
            }
        }
    });

    // Make sure we do one report immediately to populate the bar.
    let _ = tx.send(());

    // And re-report whenever anything of interest may have changed.
    let mut event_listener = EventListener::new();
    let txx = tx.clone();
    event_listener.add_workspace_change_handler(move |_| {
        let _ = txx.send(());
    });
    let txx = tx.clone();
    event_listener.add_window_moved_handler(move |_| {
        let _ = txx.send(());
    });
    let txx = tx.clone();
    event_listener.add_window_open_handler(move |_| {
        let _ = txx.send(());
    });
    let txx = tx.clone();
    event_listener.add_window_close_handler(move |_| {
        let _ = txx.send(());
    });

    drop(tx);
    event_listener
        .start_listener()
        .context("start event listener")
}
