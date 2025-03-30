use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models;

pub async fn test_user_repository_save_once<R>(repo: R)
where
    R: UserRepository + Send + Sync,
{
    let user = models::User::new(42, Some("username".to_string()));
    repo.save(user.clone()).await.unwrap();
    
    let fetched = repo.user(42).await.unwrap();
    assert_eq!(user, fetched);
}

pub async fn test_user_repository_save_twice<R>(repo: R)
where
    R: UserRepository + Send + Sync,
{
    let user = models::User::new(42, Some("username".to_string()));
    
    repo.save(user.clone()).await.unwrap();
    
    let res = repo.save(user.clone()).await;
    assert!(res.is_err());
    let err = res.err().unwrap();

    let expected = DomainError::AlreadyExists("user already exists: 42".to_string());
    assert!(matches!(err, expected));

}

pub async fn test_user_repository_get_not_found<R>(repo: R)
where
    R: UserRepository + Send + Sync,
{
    let res = repo.user(42).await;
    assert!(res.is_err());
    let err = res.err().unwrap();

    let expected = DomainError::NotFound("user not found: 42".to_string());
    assert!(matches!(err, expected));
}
