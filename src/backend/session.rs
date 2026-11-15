use super::event::Event;

#[derive(Clone, Default, PartialEq, Debug)]
pub enum Session {
    #[default]
    Starting,
    Pairing(String),
    Online { offline: bool },
    Failed(String),
}

impl Session {
    pub fn next(self, event: Event) -> Session {
        match (self, event) {
            (_, Event::Qr(code)) => Session::Pairing(code),
            (_, Event::Connected) => Session::Online { offline: false },
            (_, Event::LoggedOut) => Session::Starting,
            (Session::Online { .. }, Event::Offline) => Session::Online { offline: true },
            (session, Event::Offline) => session,
            (_, Event::Failed(reason)) => Session::Failed(reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanning_the_code_goes_online() {
        let s = Session::Starting.next(Event::Qr("a".into())).next(Event::Qr("b".into())).next(Event::Connected);
        assert_eq!(s, Session::Online { offline: false });
    }

    #[test]
    fn reconnecting_clears_the_offline_flag() {
        let s = Session::Online { offline: false }.next(Event::Offline).next(Event::Connected);
        assert_eq!(s, Session::Online { offline: false });
    }

    #[test]
    fn dropping_only_marks_an_online_session_offline() {
        assert_eq!(Session::Online { offline: false }.next(Event::Offline), Session::Online { offline: true });
        assert_eq!(Session::Starting.next(Event::Offline), Session::Starting);
        assert_eq!(Session::Pairing("a".into()).next(Event::Offline), Session::Pairing("a".into()));
    }

    #[test]
    fn a_logout_starts_over() {
        assert_eq!(Session::Online { offline: false }.next(Event::LoggedOut), Session::Starting);
    }

    #[test]
    fn a_failure_is_kept_until_something_else_happens() {
        let s = Session::Starting.next(Event::Failed("db".into()));
        assert_eq!(s, Session::Failed("db".into()));
        assert_eq!(s.next(Event::Qr("a".into())), Session::Pairing("a".into()));
    }
}
