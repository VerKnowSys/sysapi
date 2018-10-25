/*
cell.rs
sentry.rs
igniter.rs
zone.rs
proxy.rs
status.rs
snapshot.rs
rollback.rs
*/


pub mod sentry {

    #[derive(Debug, Serialize)]
    pub struct Sentry {
        cell_name: String,

    }

}


pub mod igniter {

    #[derive(Debug, Serialize)]
    enum Actions {
        Start,
        Stop,
        Restart,
        Reload,
        Test,
        Watch,
    }


    #[derive(Debug, Serialize)]
    pub struct Igniter {
        service_name: String,
        action: Actions,
    }

}


pub mod zone {

    #[derive(Debug, Serialize)]
    enum ZoneTypes {
        A_,
        Cname,
        Txt,
    }

    #[derive(Debug, Serialize)]
    pub struct Zone {
        domain_name: String,
        ipv4: String,
        zone_type: ZoneTypes,
    }

}


pub mod proxy {

    #[derive(Debug, Serialize)]
    pub struct Proxy {
        from: String,
        to: String,
    }

}


pub mod status {

    #[derive(Debug, Serialize)]
    pub struct Status {
        service_name: String,
    }

}


pub mod snapshot {

    #[derive(Debug, Serialize)]
    pub struct Snapshot {
        name: String,
        dataset_path: String,
    }

}


pub mod rollback {

    #[derive(Debug, Serialize)]
    pub struct Rollback {
        name: String,
        dataset_path: String,
    }

}
