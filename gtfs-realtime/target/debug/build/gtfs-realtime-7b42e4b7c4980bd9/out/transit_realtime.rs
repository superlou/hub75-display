/// The contents of a feed message.
/// A feed is a continuous stream of feed messages. Each message in the stream is
/// obtained as a response to an appropriate HTTP GET request.
/// A realtime feed is always defined with relation to an existing GTFS feed.
/// All the entity ids are resolved with respect to the GTFS feed.
/// Note that "required" and "optional" as stated in this file refer to Protocol
/// Buffer cardinality, not semantic cardinality.  See reference.md at
/// <https://github.com/google/transit/tree/master/gtfs-realtime> for field
/// semantic cardinality.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMessage {
    /// Metadata about this feed and feed message.
    #[prost(message, required, tag = "1")]
    pub header: FeedHeader,
    /// Contents of the feed.
    #[prost(message, repeated, tag = "2")]
    pub entity: ::prost::alloc::vec::Vec<FeedEntity>,
}
/// Metadata about a feed, included in feed messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedHeader {
    /// Version of the feed specification.
    /// The current version is 2.0.
    #[prost(string, required, tag = "1")]
    pub gtfs_realtime_version: ::prost::alloc::string::String,
    #[prost(
        enumeration = "feed_header::Incrementality",
        optional,
        tag = "2",
        default = "FullDataset"
    )]
    pub incrementality: ::core::option::Option<i32>,
    /// This timestamp identifies the moment when the content of this feed has been
    /// created (in server time). In POSIX time (i.e., number of seconds since
    /// January 1st 1970 00:00:00 UTC).
    #[prost(uint64, optional, tag = "3")]
    pub timestamp: ::core::option::Option<u64>,
}
/// Nested message and enum types in `FeedHeader`.
pub mod feed_header {
    /// Determines whether the current fetch is incremental.  Currently,
    /// DIFFERENTIAL mode is unsupported and behavior is unspecified for feeds
    /// that use this mode.  There are discussions on the GTFS Realtime mailing
    /// list around fully specifying the behavior of DIFFERENTIAL mode and the
    /// documentation will be updated when those discussions are finalized.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Incrementality {
        FullDataset = 0,
        Differential = 1,
    }
    impl Incrementality {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Incrementality::FullDataset => "FULL_DATASET",
                Incrementality::Differential => "DIFFERENTIAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FULL_DATASET" => Some(Self::FullDataset),
                "DIFFERENTIAL" => Some(Self::Differential),
                _ => None,
            }
        }
    }
}
/// A definition (or update) of an entity in the transit feed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedEntity {
    /// The ids are used only to provide incrementality support. The id should be
    /// unique within a FeedMessage. Consequent FeedMessages may contain
    /// FeedEntities with the same id. In case of a DIFFERENTIAL update the new
    /// FeedEntity with some id will replace the old FeedEntity with the same id
    /// (or delete it - see is_deleted below).
    /// The actual GTFS entities (e.g. stations, routes, trips) referenced by the
    /// feed must be specified by explicit selectors (see EntitySelector below for
    /// more info).
    #[prost(string, required, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Whether this entity is to be deleted. Relevant only for incremental
    /// fetches.
    #[prost(bool, optional, tag = "2", default = "false")]
    pub is_deleted: ::core::option::Option<bool>,
    /// Data about the entity itself. Exactly one of the following fields must be
    /// present (unless the entity is being deleted).
    #[prost(message, optional, tag = "3")]
    pub trip_update: ::core::option::Option<TripUpdate>,
    #[prost(message, optional, tag = "4")]
    pub vehicle: ::core::option::Option<VehiclePosition>,
    #[prost(message, optional, tag = "5")]
    pub alert: ::core::option::Option<Alert>,
}
/// Realtime update of the progress of a vehicle along a trip.
/// Depending on the value of ScheduleRelationship, a TripUpdate can specify:
/// - A trip that proceeds along the schedule.
/// - A trip that proceeds along a route but has no fixed schedule.
/// - A trip that have been added or removed with regard to schedule.
///
/// The updates can be for future, predicted arrival/departure events, or for
/// past events that already occurred.
/// Normally, updates should get more precise and more certain (see
/// uncertainty below) as the events gets closer to current time.
/// Even if that is not possible, the information for past events should be
/// precise and certain. In particular, if an update points to time in the past
/// but its update's uncertainty is not 0, the client should conclude that the
/// update is a (wrong) prediction and that the trip has not completed yet.
///
/// Note that the update can describe a trip that is already completed.
/// To this end, it is enough to provide an update for the last stop of the trip.
/// If the time of that is in the past, the client will conclude from that that
/// the whole trip is in the past (it is possible, although inconsequential, to
/// also provide updates for preceding stops).
/// This option is most relevant for a trip that has completed ahead of schedule,
/// but according to the schedule, the trip is still proceeding at the current
/// time. Removing the updates for this trip could make the client assume
/// that the trip is still proceeding.
/// Note that the feed provider is allowed, but not required, to purge past
/// updates - this is one case where this would be practically useful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripUpdate {
    /// The Trip that this message applies to. There can be at most one
    /// TripUpdate entity for each actual trip instance.
    /// If there is none, that means there is no prediction information available.
    /// It does *not* mean that the trip is progressing according to schedule.
    #[prost(message, required, tag = "1")]
    pub trip: TripDescriptor,
    /// Additional information on the vehicle that is serving this trip.
    #[prost(message, optional, tag = "3")]
    pub vehicle: ::core::option::Option<VehicleDescriptor>,
    /// Updates to StopTimes for the trip (both future, i.e., predictions, and in
    /// some cases, past ones, i.e., those that already happened).
    /// The updates must be sorted by stop_sequence, and apply for all the
    /// following stops of the trip up to the next specified one.
    ///
    /// Example 1:
    /// For a trip with 20 stops, a StopTimeUpdate with arrival delay and departure
    /// delay of 0 for stop_sequence of the current stop means that the trip is
    /// exactly on time.
    ///
    /// Example 2:
    /// For the same trip instance, 3 StopTimeUpdates are provided:
    /// - delay of 5 min for stop_sequence 3
    /// - delay of 1 min for stop_sequence 8
    /// - delay of unspecified duration for stop_sequence 10
    /// This will be interpreted as:
    /// - stop_sequences 3,4,5,6,7 have delay of 5 min.
    /// - stop_sequences 8,9 have delay of 1 min.
    /// - stop_sequences 10,... have unknown delay.
    #[prost(message, repeated, tag = "2")]
    pub stop_time_update: ::prost::alloc::vec::Vec<trip_update::StopTimeUpdate>,
    /// Moment at which the vehicle's real-time progress was measured. In POSIX
    /// time (i.e., the number of seconds since January 1st 1970 00:00:00 UTC).
    #[prost(uint64, optional, tag = "4")]
    pub timestamp: ::core::option::Option<u64>,
    /// The current schedule deviation for the trip.  Delay should only be
    /// specified when the prediction is given relative to some existing schedule
    /// in GTFS.
    ///
    /// Delay (in seconds) can be positive (meaning that the vehicle is late) or
    /// negative (meaning that the vehicle is ahead of schedule). Delay of 0
    /// means that the vehicle is exactly on time.
    ///
    /// Delay information in StopTimeUpdates take precedent of trip-level delay
    /// information, such that trip-level delay is only propagated until the next
    /// stop along the trip with a StopTimeUpdate delay value specified.
    ///
    /// Feed providers are strongly encouraged to provide a TripUpdate.timestamp
    /// value indicating when the delay value was last updated, in order to
    /// evaluate the freshness of the data.
    ///
    /// NOTE: This field is still experimental, and subject to change. It may be
    /// formally adopted in the future.
    #[prost(int32, optional, tag = "5")]
    pub delay: ::core::option::Option<i32>,
}
/// Nested message and enum types in `TripUpdate`.
pub mod trip_update {
    /// Timing information for a single predicted event (either arrival or
    /// departure).
    /// Timing consists of delay and/or estimated time, and uncertainty.
    /// - delay should be used when the prediction is given relative to some
    ///    existing schedule in GTFS.
    /// - time should be given whether there is a predicted schedule or not. If
    ///    both time and delay are specified, time will take precedence
    ///    (although normally, time, if given for a scheduled trip, should be
    ///    equal to scheduled time in GTFS + delay).
    ///
    /// Uncertainty applies equally to both time and delay.
    /// The uncertainty roughly specifies the expected error in true delay (but
    /// note, we don't yet define its precise statistical meaning). It's possible
    /// for the uncertainty to be 0, for example for trains that are driven under
    /// computer timing control.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StopTimeEvent {
        /// Delay (in seconds) can be positive (meaning that the vehicle is late) or
        /// negative (meaning that the vehicle is ahead of schedule). Delay of 0
        /// means that the vehicle is exactly on time.
        #[prost(int32, optional, tag = "1")]
        pub delay: ::core::option::Option<i32>,
        /// Event as absolute time.
        /// In Unix time (i.e., number of seconds since January 1st 1970 00:00:00
        /// UTC).
        #[prost(int64, optional, tag = "2")]
        pub time: ::core::option::Option<i64>,
        /// If uncertainty is omitted, it is interpreted as unknown.
        /// If the prediction is unknown or too uncertain, the delay (or time) field
        /// should be empty. In such case, the uncertainty field is ignored.
        /// To specify a completely certain prediction, set its uncertainty to 0.
        #[prost(int32, optional, tag = "3")]
        pub uncertainty: ::core::option::Option<i32>,
    }
    /// Realtime update for arrival and/or departure events for a given stop on a
    /// trip. Updates can be supplied for both past and future events.
    /// The producer is allowed, although not required, to drop past events.
    ///
    /// The update is linked to a specific stop either through stop_sequence or
    /// stop_id, so one of the fields below must necessarily be set.
    /// See the documentation in TripDescriptor for more information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StopTimeUpdate {
        /// Must be the same as in stop_times.txt in the corresponding GTFS feed.
        #[prost(uint32, optional, tag = "1")]
        pub stop_sequence: ::core::option::Option<u32>,
        /// Must be the same as in stops.txt in the corresponding GTFS feed.
        #[prost(string, optional, tag = "4")]
        pub stop_id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "2")]
        pub arrival: ::core::option::Option<StopTimeEvent>,
        #[prost(message, optional, tag = "3")]
        pub departure: ::core::option::Option<StopTimeEvent>,
        #[prost(
            enumeration = "stop_time_update::ScheduleRelationship",
            optional,
            tag = "5",
            default = "Scheduled"
        )]
        pub schedule_relationship: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `StopTimeUpdate`.
    pub mod stop_time_update {
        /// The relation between this StopTime and the static schedule.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ScheduleRelationship {
            /// The vehicle is proceeding in accordance with its static schedule of
            /// stops, although not necessarily according to the times of the schedule.
            /// At least one of arrival and departure must be provided. If the schedule
            /// for this stop contains both arrival and departure times then so must
            /// this update.
            Scheduled = 0,
            /// The stop is skipped, i.e., the vehicle will not stop at this stop.
            /// Arrival and departure are optional.
            Skipped = 1,
            /// No data is given for this stop. The main intention for this value is to
            /// give the predictions only for part of a trip, i.e., if the last update
            /// for a trip has a NO_DATA specifier, then StopTimes for the rest of the
            /// stops in the trip are considered to be unspecified as well.
            /// Neither arrival nor departure should be supplied.
            NoData = 2,
        }
        impl ScheduleRelationship {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ScheduleRelationship::Scheduled => "SCHEDULED",
                    ScheduleRelationship::Skipped => "SKIPPED",
                    ScheduleRelationship::NoData => "NO_DATA",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SCHEDULED" => Some(Self::Scheduled),
                    "SKIPPED" => Some(Self::Skipped),
                    "NO_DATA" => Some(Self::NoData),
                    _ => None,
                }
            }
        }
    }
}
/// Realtime positioning information for a given vehicle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehiclePosition {
    /// The Trip that this vehicle is serving.
    /// Can be empty or partial if the vehicle can not be identified with a given
    /// trip instance.
    #[prost(message, optional, tag = "1")]
    pub trip: ::core::option::Option<TripDescriptor>,
    /// Additional information on the vehicle that is serving this trip.
    #[prost(message, optional, tag = "8")]
    pub vehicle: ::core::option::Option<VehicleDescriptor>,
    /// Current position of this vehicle.
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<Position>,
    /// The stop sequence index of the current stop. The meaning of
    /// current_stop_sequence (i.e., the stop that it refers to) is determined by
    /// current_status.
    /// If current_status is missing IN_TRANSIT_TO is assumed.
    #[prost(uint32, optional, tag = "3")]
    pub current_stop_sequence: ::core::option::Option<u32>,
    /// Identifies the current stop. The value must be the same as in stops.txt in
    /// the corresponding GTFS feed.
    #[prost(string, optional, tag = "7")]
    pub stop_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The exact status of the vehicle with respect to the current stop.
    /// Ignored if current_stop_sequence is missing.
    #[prost(
        enumeration = "vehicle_position::VehicleStopStatus",
        optional,
        tag = "4",
        default = "InTransitTo"
    )]
    pub current_status: ::core::option::Option<i32>,
    /// Moment at which the vehicle's position was measured. In POSIX time
    /// (i.e., number of seconds since January 1st 1970 00:00:00 UTC).
    #[prost(uint64, optional, tag = "5")]
    pub timestamp: ::core::option::Option<u64>,
    #[prost(enumeration = "vehicle_position::CongestionLevel", optional, tag = "6")]
    pub congestion_level: ::core::option::Option<i32>,
    #[prost(enumeration = "vehicle_position::OccupancyStatus", optional, tag = "9")]
    pub occupancy_status: ::core::option::Option<i32>,
}
/// Nested message and enum types in `VehiclePosition`.
pub mod vehicle_position {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum VehicleStopStatus {
        /// The vehicle is just about to arrive at the stop (on a stop
        /// display, the vehicle symbol typically flashes).
        IncomingAt = 0,
        /// The vehicle is standing at the stop.
        StoppedAt = 1,
        /// The vehicle has departed and is in transit to the next stop.
        InTransitTo = 2,
    }
    impl VehicleStopStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VehicleStopStatus::IncomingAt => "INCOMING_AT",
                VehicleStopStatus::StoppedAt => "STOPPED_AT",
                VehicleStopStatus::InTransitTo => "IN_TRANSIT_TO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INCOMING_AT" => Some(Self::IncomingAt),
                "STOPPED_AT" => Some(Self::StoppedAt),
                "IN_TRANSIT_TO" => Some(Self::InTransitTo),
                _ => None,
            }
        }
    }
    /// Congestion level that is affecting this vehicle.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CongestionLevel {
        UnknownCongestionLevel = 0,
        RunningSmoothly = 1,
        StopAndGo = 2,
        Congestion = 3,
        /// People leaving their cars.
        SevereCongestion = 4,
    }
    impl CongestionLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CongestionLevel::UnknownCongestionLevel => "UNKNOWN_CONGESTION_LEVEL",
                CongestionLevel::RunningSmoothly => "RUNNING_SMOOTHLY",
                CongestionLevel::StopAndGo => "STOP_AND_GO",
                CongestionLevel::Congestion => "CONGESTION",
                CongestionLevel::SevereCongestion => "SEVERE_CONGESTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_CONGESTION_LEVEL" => Some(Self::UnknownCongestionLevel),
                "RUNNING_SMOOTHLY" => Some(Self::RunningSmoothly),
                "STOP_AND_GO" => Some(Self::StopAndGo),
                "CONGESTION" => Some(Self::Congestion),
                "SEVERE_CONGESTION" => Some(Self::SevereCongestion),
                _ => None,
            }
        }
    }
    /// The degree of passenger occupancy of the vehicle. This field is still
    /// experimental, and subject to change. It may be formally adopted in the
    /// future.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OccupancyStatus {
        /// The vehicle is considered empty by most measures, and has few or no
        /// passengers onboard, but is still accepting passengers.
        Empty = 0,
        /// The vehicle has a relatively large percentage of seats available.
        /// What percentage of free seats out of the total seats available is to be
        /// considered large enough to fall into this category is determined at the
        /// discretion of the producer.
        ManySeatsAvailable = 1,
        /// The vehicle has a relatively small percentage of seats available.
        /// What percentage of free seats out of the total seats available is to be
        /// considered small enough to fall into this category is determined at the
        /// discretion of the feed producer.
        FewSeatsAvailable = 2,
        /// The vehicle can currently accommodate only standing passengers.
        StandingRoomOnly = 3,
        /// The vehicle can currently accommodate only standing passengers
        /// and has limited space for them.
        CrushedStandingRoomOnly = 4,
        /// The vehicle is considered full by most measures, but may still be
        /// allowing passengers to board.
        Full = 5,
        /// The vehicle is not accepting additional passengers.
        NotAcceptingPassengers = 6,
    }
    impl OccupancyStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OccupancyStatus::Empty => "EMPTY",
                OccupancyStatus::ManySeatsAvailable => "MANY_SEATS_AVAILABLE",
                OccupancyStatus::FewSeatsAvailable => "FEW_SEATS_AVAILABLE",
                OccupancyStatus::StandingRoomOnly => "STANDING_ROOM_ONLY",
                OccupancyStatus::CrushedStandingRoomOnly => "CRUSHED_STANDING_ROOM_ONLY",
                OccupancyStatus::Full => "FULL",
                OccupancyStatus::NotAcceptingPassengers => "NOT_ACCEPTING_PASSENGERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EMPTY" => Some(Self::Empty),
                "MANY_SEATS_AVAILABLE" => Some(Self::ManySeatsAvailable),
                "FEW_SEATS_AVAILABLE" => Some(Self::FewSeatsAvailable),
                "STANDING_ROOM_ONLY" => Some(Self::StandingRoomOnly),
                "CRUSHED_STANDING_ROOM_ONLY" => Some(Self::CrushedStandingRoomOnly),
                "FULL" => Some(Self::Full),
                "NOT_ACCEPTING_PASSENGERS" => Some(Self::NotAcceptingPassengers),
                _ => None,
            }
        }
    }
}
/// An alert, indicating some sort of incident in the public transit network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    /// Time when the alert should be shown to the user. If missing, the
    /// alert will be shown as long as it appears in the feed.
    /// If multiple ranges are given, the alert will be shown during all of them.
    #[prost(message, repeated, tag = "1")]
    pub active_period: ::prost::alloc::vec::Vec<TimeRange>,
    /// Entities whose users we should notify of this alert.
    #[prost(message, repeated, tag = "5")]
    pub informed_entity: ::prost::alloc::vec::Vec<EntitySelector>,
    #[prost(enumeration = "alert::Cause", optional, tag = "6", default = "UnknownCause")]
    pub cause: ::core::option::Option<i32>,
    #[prost(
        enumeration = "alert::Effect",
        optional,
        tag = "7",
        default = "UnknownEffect"
    )]
    pub effect: ::core::option::Option<i32>,
    /// The URL which provides additional information about the alert.
    #[prost(message, optional, tag = "8")]
    pub url: ::core::option::Option<TranslatedString>,
    /// Alert header. Contains a short summary of the alert text as plain-text.
    #[prost(message, optional, tag = "10")]
    pub header_text: ::core::option::Option<TranslatedString>,
    /// Full description for the alert as plain-text. The information in the
    /// description should add to the information of the header.
    #[prost(message, optional, tag = "11")]
    pub description_text: ::core::option::Option<TranslatedString>,
}
/// Nested message and enum types in `Alert`.
pub mod alert {
    /// Cause of this alert.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Cause {
        UnknownCause = 1,
        /// Not machine-representable.
        OtherCause = 2,
        TechnicalProblem = 3,
        /// Public transit agency employees stopped working.
        Strike = 4,
        /// People are blocking the streets.
        Demonstration = 5,
        Accident = 6,
        Holiday = 7,
        Weather = 8,
        Maintenance = 9,
        Construction = 10,
        PoliceActivity = 11,
        MedicalEmergency = 12,
    }
    impl Cause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Cause::UnknownCause => "UNKNOWN_CAUSE",
                Cause::OtherCause => "OTHER_CAUSE",
                Cause::TechnicalProblem => "TECHNICAL_PROBLEM",
                Cause::Strike => "STRIKE",
                Cause::Demonstration => "DEMONSTRATION",
                Cause::Accident => "ACCIDENT",
                Cause::Holiday => "HOLIDAY",
                Cause::Weather => "WEATHER",
                Cause::Maintenance => "MAINTENANCE",
                Cause::Construction => "CONSTRUCTION",
                Cause::PoliceActivity => "POLICE_ACTIVITY",
                Cause::MedicalEmergency => "MEDICAL_EMERGENCY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_CAUSE" => Some(Self::UnknownCause),
                "OTHER_CAUSE" => Some(Self::OtherCause),
                "TECHNICAL_PROBLEM" => Some(Self::TechnicalProblem),
                "STRIKE" => Some(Self::Strike),
                "DEMONSTRATION" => Some(Self::Demonstration),
                "ACCIDENT" => Some(Self::Accident),
                "HOLIDAY" => Some(Self::Holiday),
                "WEATHER" => Some(Self::Weather),
                "MAINTENANCE" => Some(Self::Maintenance),
                "CONSTRUCTION" => Some(Self::Construction),
                "POLICE_ACTIVITY" => Some(Self::PoliceActivity),
                "MEDICAL_EMERGENCY" => Some(Self::MedicalEmergency),
                _ => None,
            }
        }
    }
    /// What is the effect of this problem on the affected entity.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Effect {
        NoService = 1,
        ReducedService = 2,
        /// We don't care about INsignificant delays: they are hard to detect, have
        /// little impact on the user, and would clutter the results as they are too
        /// frequent.
        SignificantDelays = 3,
        Detour = 4,
        AdditionalService = 5,
        ModifiedService = 6,
        OtherEffect = 7,
        UnknownEffect = 8,
        StopMoved = 9,
    }
    impl Effect {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Effect::NoService => "NO_SERVICE",
                Effect::ReducedService => "REDUCED_SERVICE",
                Effect::SignificantDelays => "SIGNIFICANT_DELAYS",
                Effect::Detour => "DETOUR",
                Effect::AdditionalService => "ADDITIONAL_SERVICE",
                Effect::ModifiedService => "MODIFIED_SERVICE",
                Effect::OtherEffect => "OTHER_EFFECT",
                Effect::UnknownEffect => "UNKNOWN_EFFECT",
                Effect::StopMoved => "STOP_MOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_SERVICE" => Some(Self::NoService),
                "REDUCED_SERVICE" => Some(Self::ReducedService),
                "SIGNIFICANT_DELAYS" => Some(Self::SignificantDelays),
                "DETOUR" => Some(Self::Detour),
                "ADDITIONAL_SERVICE" => Some(Self::AdditionalService),
                "MODIFIED_SERVICE" => Some(Self::ModifiedService),
                "OTHER_EFFECT" => Some(Self::OtherEffect),
                "UNKNOWN_EFFECT" => Some(Self::UnknownEffect),
                "STOP_MOVED" => Some(Self::StopMoved),
                _ => None,
            }
        }
    }
}
/// A time interval. The interval is considered active at time 't' if 't' is
/// greater than or equal to the start time and less than the end time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeRange {
    /// Start time, in POSIX time (i.e., number of seconds since January 1st 1970
    /// 00:00:00 UTC).
    /// If missing, the interval starts at minus infinity.
    #[prost(uint64, optional, tag = "1")]
    pub start: ::core::option::Option<u64>,
    /// End time, in POSIX time (i.e., number of seconds since January 1st 1970
    /// 00:00:00 UTC).
    /// If missing, the interval ends at plus infinity.
    #[prost(uint64, optional, tag = "2")]
    pub end: ::core::option::Option<u64>,
}
/// A position.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// Degrees North, in the WGS-84 coordinate system.
    #[prost(float, required, tag = "1")]
    pub latitude: f32,
    /// Degrees East, in the WGS-84 coordinate system.
    #[prost(float, required, tag = "2")]
    pub longitude: f32,
    /// Bearing, in degrees, clockwise from North, i.e., 0 is North and 90 is East.
    /// This can be the compass bearing, or the direction towards the next stop
    /// or intermediate location.
    /// This should not be direction deduced from the sequence of previous
    /// positions, which can be computed from previous data.
    #[prost(float, optional, tag = "3")]
    pub bearing: ::core::option::Option<f32>,
    /// Odometer value, in meters.
    #[prost(double, optional, tag = "4")]
    pub odometer: ::core::option::Option<f64>,
    /// Momentary speed measured by the vehicle, in meters per second.
    #[prost(float, optional, tag = "5")]
    pub speed: ::core::option::Option<f32>,
}
/// A descriptor that identifies an instance of a GTFS trip, or all instances of
/// a trip along a route.
/// - To specify a single trip instance, the trip_id (and if necessary,
///    start_time) is set. If route_id is also set, then it should be same as one
///    that the given trip corresponds to.
/// - To specify all the trips along a given route, only the route_id should be
///    set. Note that if the trip_id is not known, then stop sequence ids in
///    TripUpdate are not sufficient, and stop_ids must be provided as well. In
///    addition, absolute arrival/departure times must be provided.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripDescriptor {
    /// The trip_id from the GTFS feed that this selector refers to.
    /// For non frequency-based trips, this field is enough to uniquely identify
    /// the trip. For frequency-based trip, start_time and start_date might also be
    /// necessary.
    #[prost(string, optional, tag = "1")]
    pub trip_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The route_id from the GTFS that this selector refers to.
    #[prost(string, optional, tag = "5")]
    pub route_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The direction_id from the GTFS feed trips.txt file, indicating the
    /// direction of travel for trips this selector refers to. This field is
    /// still experimental, and subject to change. It may be formally adopted in
    /// the future.
    #[prost(uint32, optional, tag = "6")]
    pub direction_id: ::core::option::Option<u32>,
    /// The initially scheduled start time of this trip instance.
    /// When the trip_id corresponds to a non-frequency-based trip, this field
    /// should either be omitted or be equal to the value in the GTFS feed. When
    /// the trip_id corresponds to a frequency-based trip, the start_time must be
    /// specified for trip updates and vehicle positions. If the trip corresponds
    /// to exact_times=1 GTFS record, then start_time must be some multiple
    /// (including zero) of headway_secs later than frequencies.txt start_time for
    /// the corresponding time period. If the trip corresponds to exact_times=0,
    /// then its start_time may be arbitrary, and is initially expected to be the
    /// first departure of the trip. Once established, the start_time of this
    /// frequency-based trip should be considered immutable, even if the first
    /// departure time changes -- that time change may instead be reflected in a
    /// StopTimeUpdate.
    /// Format and semantics of the field is same as that of
    /// GTFS/frequencies.txt/start_time, e.g., 11:15:35 or 25:15:35.
    #[prost(string, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The scheduled start date of this trip instance.
    /// Must be provided to disambiguate trips that are so late as to collide with
    /// a scheduled trip on a next day. For example, for a train that departs 8:00
    /// and 20:00 every day, and is 12 hours late, there would be two distinct
    /// trips on the same time.
    /// This field can be provided but is not mandatory for schedules in which such
    /// collisions are impossible - for example, a service running on hourly
    /// schedule where a vehicle that is one hour late is not considered to be
    /// related to schedule anymore.
    /// In YYYYMMDD format.
    #[prost(string, optional, tag = "3")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "trip_descriptor::ScheduleRelationship", optional, tag = "4")]
    pub schedule_relationship: ::core::option::Option<i32>,
}
/// Nested message and enum types in `TripDescriptor`.
pub mod trip_descriptor {
    /// The relation between this trip and the static schedule. If a trip is done
    /// in accordance with temporary schedule, not reflected in GTFS, then it
    /// shouldn't be marked as SCHEDULED, but likely as ADDED.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ScheduleRelationship {
        /// Trip that is running in accordance with its GTFS schedule, or is close
        /// enough to the scheduled trip to be associated with it.
        Scheduled = 0,
        /// An extra trip that was added in addition to a running schedule, for
        /// example, to replace a broken vehicle or to respond to sudden passenger
        /// load.
        Added = 1,
        /// A trip that is running with no schedule associated to it, for example, if
        /// there is no schedule at all.
        Unscheduled = 2,
        /// A trip that existed in the schedule but was removed.
        Canceled = 3,
    }
    impl ScheduleRelationship {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ScheduleRelationship::Scheduled => "SCHEDULED",
                ScheduleRelationship::Added => "ADDED",
                ScheduleRelationship::Unscheduled => "UNSCHEDULED",
                ScheduleRelationship::Canceled => "CANCELED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SCHEDULED" => Some(Self::Scheduled),
                "ADDED" => Some(Self::Added),
                "UNSCHEDULED" => Some(Self::Unscheduled),
                "CANCELED" => Some(Self::Canceled),
                _ => None,
            }
        }
    }
}
/// Identification information for the vehicle performing the trip.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleDescriptor {
    /// Internal system identification of the vehicle. Should be unique per
    /// vehicle, and can be used for tracking the vehicle as it proceeds through
    /// the system.
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// User visible label, i.e., something that must be shown to the passenger to
    /// help identify the correct vehicle.
    #[prost(string, optional, tag = "2")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// The license plate of the vehicle.
    #[prost(string, optional, tag = "3")]
    pub license_plate: ::core::option::Option<::prost::alloc::string::String>,
}
/// A selector for an entity in a GTFS feed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntitySelector {
    /// The values of the fields should correspond to the appropriate fields in the
    /// GTFS feed.
    /// At least one specifier must be given. If several are given, then the
    /// matching has to apply to all the given specifiers.
    #[prost(string, optional, tag = "1")]
    pub agency_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub route_id: ::core::option::Option<::prost::alloc::string::String>,
    /// corresponds to route_type in GTFS.
    #[prost(int32, optional, tag = "3")]
    pub route_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub trip: ::core::option::Option<TripDescriptor>,
    #[prost(string, optional, tag = "5")]
    pub stop_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// An internationalized message containing per-language versions of a snippet of
/// text or a URL.
/// One of the strings from a message will be picked up. The resolution proceeds
/// as follows:
/// 1. If the UI language matches the language code of a translation,
///     the first matching translation is picked.
/// 2. If a default UI language (e.g., English) matches the language code of a
///     translation, the first matching translation is picked.
/// 3. If some translation has an unspecified language code, that translation is
///     picked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslatedString {
    /// At least one translation must be provided.
    #[prost(message, repeated, tag = "1")]
    pub translation: ::prost::alloc::vec::Vec<translated_string::Translation>,
}
/// Nested message and enum types in `TranslatedString`.
pub mod translated_string {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Translation {
        /// A UTF-8 string containing the message.
        #[prost(string, required, tag = "1")]
        pub text: ::prost::alloc::string::String,
        /// BCP-47 language code. Can be omitted if the language is unknown or if
        /// no i18n is done at all for the feed. At most one translation is
        /// allowed to have an unspecified language tag.
        #[prost(string, optional, tag = "2")]
        pub language: ::core::option::Option<::prost::alloc::string::String>,
    }
}
