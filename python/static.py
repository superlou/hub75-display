from datetime import timedelta, datetime
import polars as pl


def find_stop_id(station_name: str):
    stops = (
        pl.scan_csv("stops.txt")
        .filter(pl.col("stop_name") == station_name)
    ).collect()

    stop_id = stops[0, "stop_id"]
    return stop_id


def main():
    stop_id = find_stop_id("Mamaroneck")
    print("Stop ID:", stop_id)

    trips = pl.scan_csv("trips.txt", dtypes={
        "trip_short_name": str
    })

    stop_times = pl.scan_csv("stop_times.txt", dtypes={
        'track': str
    })

    # Metronorth doesn't appear to use calendar.txt to define daily services, and
    # instead has a unique date for each trip.
    calendar_dates = (
        pl.scan_csv("calendar_dates.txt")
        .with_columns(
            pl.col("date").cast(str).str.strptime(pl.Date, "%Y%m%d")
        )
    )

    time_cutoff = datetime.now() - pl.duration(minutes=30)

    scheduled = (
        stop_times
        .join(trips, on="trip_id")
        .join(calendar_dates, on="service_id")
        .filter(pl.col("stop_id") == stop_id)        
        .select(["trip_id", "date", "track", "arrival_time", "trip_headsign"])
        .with_columns(
            (pl.col("arrival_time").str.slice(0, 2).cast(int) * 3600 +
             pl.col("arrival_time").str.slice(3, 2).cast(int) * 60 +
             pl.col("arrival_time").str.slice(6, 2).cast(int)).alias("arrival_seconds")
        )
        .with_columns(
            (pl.col("date").cast(pl.Datetime) +
             pl.duration(seconds=pl.col("arrival_seconds"))).alias("arrival_datetime")
        )
        .filter(pl.col("arrival_datetime") > time_cutoff)
        .sort("arrival_datetime")
    )

    print(scheduled.describe_optimized_plan())
    print(scheduled.collect())


if __name__ == "__main__":
    main()
