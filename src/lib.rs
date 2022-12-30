use peppi::model::primitives::Port;

pub enum MatchedPlayers {
    OneWay(Port, Port),
    Both(Port, Port),
    NoMatch,
}
