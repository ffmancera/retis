use std::{fs::OpenOptions, time::Instant};

use anyhow::{bail, Result};

use crate::{
    core::events::{file::FileEventsFactory, *},
    module::get_modules,
    process::{display::*, series::EventSorter, tracking::AddTracking},
};

/// Benchmark time to output events (text, json).
pub(super) fn bench(ci: bool) -> Result<()> {
    let modules = get_modules()?;
    let iters = match ci {
        false => 1000000,
        true => 1,
    };

    // PrintSingle benchmark

    let mut factory = FileEventsFactory::new("test_data/test_events.json")?;
    factory.start(modules.section_factories()?)?;
    let event = match factory.next_event(None)? {
        EventResult::Event(event) => event,
        _ => bail!("Could not get event from test file"),
    };

    let mut p = PrintSingle::text(
        Box::new(OpenOptions::new().write(true).open("/dev/null")?),
        DisplayFormat::SingleLine,
    );
    let now = Instant::now();
    for _ in 0..iters {
        p.process_one(&event)?;
    }
    println!(
        "1M_print_single_singleline_us {}",
        now.elapsed().as_micros()
    );

    let mut p = PrintSingle::text(
        Box::new(OpenOptions::new().write(true).open("/dev/null")?),
        DisplayFormat::MultiLine,
    );
    let now = Instant::now();
    for _ in 0..iters {
        p.process_one(&event)?;
    }
    println!("1M_print_single_multiline_us {}", now.elapsed().as_micros());

    let mut p = PrintSingle::json(Box::new(OpenOptions::new().write(true).open("/dev/null")?));
    let now = Instant::now();
    for _ in 0..iters {
        p.process_one(&event)?;
    }
    println!("1M_print_single_json_us {}", now.elapsed().as_micros());

    // PrintSeries benchmark

    let mut factory = FileEventsFactory::new("test_data/test_events.json")?;
    factory.start(modules.section_factories()?)?;
    let mut tracker = AddTracking::new();
    let mut series = EventSorter::new();

    while let EventResult::Event(mut event) = factory.next_event(None)? {
        tracker.process_one(&mut event)?;
        series.add(event);
    }
    let series = series.pop_oldest()?.unwrap();

    let mut p = PrintSeries::text(
        Box::new(OpenOptions::new().write(true).open("/dev/null")?),
        DisplayFormat::SingleLine,
    );
    let now = Instant::now();
    for _ in 0..iters {
        p.process_one(&series)?;
    }
    println!(
        "1M_print_series_singleline_us {}",
        now.elapsed().as_micros()
    );

    let mut p = PrintSeries::text(
        Box::new(OpenOptions::new().write(true).open("/dev/null")?),
        DisplayFormat::MultiLine,
    );
    let now = Instant::now();
    for _ in 0..iters {
        p.process_one(&series)?;
    }
    println!("1M_print_series_multiline_us {}", now.elapsed().as_micros());

    let mut p = PrintSeries::json(Box::new(OpenOptions::new().write(true).open("/dev/null")?));
    let now = Instant::now();
    for _ in 0..iters {
        p.process_one(&series)?;
    }
    println!("1M_print_series_json_us {}", now.elapsed().as_micros());

    Ok(())
}
