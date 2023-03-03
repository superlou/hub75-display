from pprint import pprint
from datetime import datetime
from dotenv import load_dotenv
from realtime import get_feed_message, trip_updates_for_stop
from static import find_stop_id, prepare_stop_data
import polars as pl


def next_x_updates(updates, count):
    updates.sort(key=lambda update: update["updates"][0]["arrival"])
    updates = [update for update in updates
               if update["updates"][0]["arrival"] >= datetime.now()]
    return updates[:3]


def main():
    stop_id = find_stop_id("Mamaroneck")

    load_dotenv("../.env")
    feed_message = get_feed_message()
    updates = trip_updates_for_stop(feed_message, str(stop_id))
    updates = next_x_updates(updates, 3)

    static_data = prepare_stop_data()
    query = (
        static_data
            .filter(pl.col("date") == datetime.now().date())
            .filter(pl.col("stop_id") == stop_id)
            .sort("arrival_time")
    )

    for update in updates:
        vehicle_label = update["vehicle_label"]
        print(query.filter(pl.col("trip_short_name") == vehicle_label).collect())
        pprint(update) 


if __name__ == "__main__":
    main()