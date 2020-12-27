use warp::Filter;

pub fn with_service<T>(
    service: T,
) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone
where
    T: Clone + std::marker::Send,
{
    warp::any().map(move || service.clone())
}
