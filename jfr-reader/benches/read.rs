// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{criterion_group, criterion_main, Criterion};
use jfr_reader::{
    chunk::{ChunkReader, SliceReader},
    chunk_event::ChunkEvent,
    error::Result,
    recording::FileReader,
    types::openjdk17,
};
use std::{io::Cursor, time::Duration};

fn read_chunks(data: &[u8]) -> Result<()> {
    let mut fr = FileReader::from_stream(Cursor::new(data))?;

    while let Some(_) = fr.next_chunk_data()? {}

    Ok(())
}

fn read_resolver(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(chunk)?.1;
        let _ = reader.resolver()?;
    }

    Ok(())
}

fn read_constant_pool_values(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;
        let resolver = reader.resolver()?;
        let _ = resolver.constant_pool_values()?;
    }

    Ok(())
}

fn read_metadata(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;
        let _ = reader.metadata()?;
    }

    Ok(())
}

fn iter_event_records(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        for er in reader.iter_event_records() {
            er?;
        }
    }

    Ok(())
}

fn event_records_sorted(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        let _ = reader.event_records_sorted()?;
    }

    Ok(())
}

fn events_fields_data(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        for er in reader.iter_event_records() {
            let _ = er?.fields_data()?;
        }
    }

    Ok(())
}

fn events_start_duration(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        for er in reader.iter_event_records() {
            let _ = er?.start_duration()?;
        }
    }

    Ok(())
}

fn events_date_time(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        let resolver = reader.resolver()?;
        let tr = resolver.time_resolver();

        for er in reader.iter_event_records() {
            let er = er?;

            tr.date_time(er.start_ticks()?);
        }
    }

    Ok(())
}

fn events_counts(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        let _ = reader.event_counts(false)?;
    }

    Ok(())
}

fn events_counts_with_names(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        let _ = reader.event_counts(true)?;
    }

    Ok(())
}

fn events_value(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        let resolver = reader.resolver()?;

        for er in reader.iter_event_records() {
            let er = er?;

            if er.is_special_event() {
                continue;
            }

            let _ = er.resolve_value(&resolver)?;
        }
    }

    Ok(())
}

fn events_resolve_enum(chunks: &[Vec<u8>]) -> Result<()> {
    for chunk in chunks {
        let reader = SliceReader::new(&chunk)?.1;

        let resolver = reader.resolver()?;
        let cpv = resolver.constant_pool_values()?;

        for er in reader.iter_event_records() {
            let er = er?;

            if er.is_special_event() {
                continue;
            }

            let v = er.resolve_value(&resolver)?;
            let _ = v.deserialize_enum::<openjdk17::Events>(&cpv);
        }
    }

    Ok(())
}

pub fn bench_intellij(c: &mut Criterion) {
    let mut group = c.benchmark_group("intellij.jfr");

    let input = include_bytes!("../testdata/intellij.jfr.zst");
    let input = zstd::decode_all(input.as_slice()).unwrap();

    let mut fr = FileReader::from_stream(Cursor::new(input.as_slice())).unwrap();
    let chunks = fr.all_chunks().unwrap();

    // Now we can perform benchmarks with this group
    group.bench_function("read-chunks", |b| {
        b.iter(|| read_chunks(input.as_slice()).unwrap())
    });

    group.bench_function("read-resolver", |b| {
        b.iter(|| read_resolver(&chunks).unwrap())
    });
    group.bench_function("read-constant-pool-values", |b| {
        b.iter(|| read_constant_pool_values(&chunks).unwrap())
    });
    group.bench_function("read-metadata", |b| {
        b.iter(|| read_metadata(&chunks).unwrap())
    });
    group.bench_function("iter-event-records", |b| {
        b.iter(|| iter_event_records(&chunks).unwrap())
    });
    group.bench_function("event-records-sorted", |b| {
        b.iter(|| event_records_sorted(&chunks).unwrap())
    });
    group.bench_function("events-fields-data", |b| {
        b.iter(|| events_fields_data(&chunks).unwrap())
    });
    group.bench_function("events-start-duration", |b| {
        b.iter(|| events_start_duration(&chunks).unwrap())
    });
    group.bench_function("events-date-time", |b| {
        b.iter(|| events_date_time(&chunks).unwrap())
    });
    group.bench_function("events-counts-no-names", |b| {
        b.iter(|| events_counts(&chunks).unwrap())
    });
    group.bench_function("events-counts-with-names", |b| {
        b.iter(|| events_counts_with_names(&chunks).unwrap())
    });
    group.bench_function("events-value", |b| {
        b.iter(|| events_value(&chunks).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("intellij.jfr Long");
    group.warm_up_time(Duration::from_millis(500));
    group.sample_size(10);

    group.bench_function("events-resolve-enum", |b| {
        b.iter(|| events_resolve_enum(&chunks).unwrap())
    });

    group.finish();
}

criterion_group! {
    name = intellij;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2));
    targets = bench_intellij,
}
criterion_main!(intellij);
