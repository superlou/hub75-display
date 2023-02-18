import os
from datetime import datetime
from pprint import pprint
import requests
from dotenv import load_dotenv
import gtfs_realtime_pb2 as gtfs_rt


def trip_updates_for_stop(feed_message, stop_id):
    trips = []

    trip_updates = [feed_entity.trip_update for feed_entity in feed_message.entity if feed_entity.HasField("trip_update")]

    for trip_update in trip_updates:
        stop_time_updates = [stu for stu in trip_update.stop_time_update if stu.stop_id == stop_id]

        if len(stop_time_updates) > 0:
            trip = {
                "trip_id": trip_update.trip.trip_id,
                "route_id": trip_update.trip.route_id,
                "direction_id": trip_update.trip.direction_id,
                "start_time": trip_update.trip.start_time,
                "start_date": trip_update.trip.start_date,
                "schedule_relationship": gtfs_rt.TripDescriptor.ScheduleRelationship.Name(trip_update.trip.schedule_relationship),
                "updates": [
                    {
                        "arrival": datetime.fromtimestamp(stu.arrival.time),
                        "delay": stu.arrival.delay
                    }
                    for stu in stop_time_updates
                ]
            }

            trips.append(trip)

    return trips


def main():
    load_dotenv("../.env")

    mnr_realtime_endpoint = (
        "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/mnr%2Fgtfs-mnr"
    )

    headers = {
        "x-api-key": os.getenv("MTA_API_KEY")
    }
    response = requests.get(mnr_realtime_endpoint, headers=headers)
    feed_msg = gtfs_rt.FeedMessage()
    feed_msg.ParseFromString(response.content)
    
    # print(msg)

    trips_updates = []

    print(feed_msg.header)
    print(f"{len(feed_msg.entity)} entities in FeedMessage")
    trip_updates = trip_updates_for_stop(feed_msg, "111")
    pprint(trip_updates)


if __name__ == "__main__":
    main()
