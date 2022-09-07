#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::codec::{Decode, Encode};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::string::String;
	use scale_info::prelude::vec;
	use scale_info::prelude::vec::Vec;
	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct LatLong {
		lat: u32,
		long: u32,
	}
	impl Default for LatLong {
		fn default() -> Self {
			Self { lat: 0, long: 0 }
		}
	}
	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Comment<UserCommentId> {
		account_id: Option<UserCommentId>,
		comment: Vec<u8>,
	}
	impl<UserCommentId> Default for Comment<UserCommentId> {
		fn default() -> Self {
			Self { account_id: None, comment: b"".to_vec() }
		}
	}
	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Address {
		address_line_1: Vec<u8>,
		address_line_2: Vec<u8>,
		google_place_id: Vec<u8>,
		city: Vec<u8>,
		state: Vec<u8>,
		country: Vec<u8>,
		location: LatLong,
	}
	impl Default for Address {
		fn default() -> Self {
			Self {
				address_line_1: b"".to_vec(),
				address_line_2: b"".to_vec(),
				google_place_id: b"".to_vec(),
				city: b"".to_vec(),
				state: b"".to_vec(),
				country: b"".to_vec(),
				location: LatLong { lat: 0, long: 0 },
			}
		}
	}

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct User<UserAccountid> {
		name: Vec<u8>,
		age: u8,
		image: Vec<u8>,
		about: Vec<u8>,
		tag: Vec<u8>,
		address: Address,
		followers: Vec<UserAccountid>,
		following: Vec<UserAccountid>,
		bookmark: Vec<u32>,
		home_time_line: Vec<u32>,
		public_time_line: Vec<u32>,
	}
	impl<UserAccountid> Default for User<UserAccountid> {
		fn default() -> Self {
			Self {
				name: b"".to_vec(),
				age: 0,
				image: b"".to_vec(),
				about: b"".to_vec(),
				tag: b"".to_vec(),
				address: Address {
					address_line_1: b"".to_vec(),
					address_line_2: b"".to_vec(),
					google_place_id: b"".to_vec(),
					city: b"".to_vec(),
					state: b"".to_vec(),
					country: b"".to_vec(),
					location: LatLong { lat: 0, long: 0 },
				},
				followers: vec![],
				following: vec![],
				bookmark: vec![],
				home_time_line: vec![],
				public_time_line: vec![],
			}
		}
	}

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Tweet<TwAccountid> {
		id: u32,
		text: Vec<u8>,
		image: Vec<u8>,
		account_id: Option<TwAccountid>,
		likes: u32,
		user_liked_tweet: Vec<TwAccountid>,
		comments: Vec<Comment<TwAccountid>>,
		tag: Vec<u8>,
	}
	impl<TwAccountid> Default for Tweet<TwAccountid> {
		fn default() -> Self {
			Self {
				id: 0,
				text: b"".to_vec(),
				image: b"".to_vec(),
				account_id: None,
				likes: 0,
				user_liked_tweet: vec![],
				tag: b"".to_vec(),
				comments: vec![],
			}
		}
	}
	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn tweets)]
	pub type TweetsStore<T> = StorageValue<_, Vec<Tweet<AccountOf<T>>>>;

	#[pallet::storage]
	#[pallet::getter(fn user)]
	pub(super) type UserStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<AccountOf<T>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserStored(User<AccountOf<T>>, T::AccountId),
		UserTimeLine(Vec<Tweet<AccountOf<T>>>, T::AccountId),
		UserLikedATweet(u32, T::AccountId),
		UserBookmarkedTweet(u32, T::AccountId),
		UserBookmarkedTweets(Vec<u32>, T::AccountId),
		UserSearchTweetByTag(Vec<Tweet<AccountOf<T>>>, T::AccountId),
		UserCommentedOnTweet(Comment<AccountOf<T>>, T::AccountId),
		TweetStored(Tweet<AccountOf<T>>, T::AccountId),
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_user(origin: OriginFor<T>, data: User<AccountOf<T>>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<UserStore<T>>::insert(&who, data.clone());

			Self::deposit_event(Event::UserStored(data, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn like_a_tweet(origin: OriginFor<T>, tweet_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			let mut temp_tweets = tweets.clone();

			let mut total_likes = 0;
			for (index, tweet) in tweets.iter().enumerate() {
				let mut tw = tweet.clone();
				if tw.id == tweet_id {
					total_likes = tw.likes + 1;
					tw.likes = total_likes;
					tw.user_liked_tweet.push(who.clone());
					temp_tweets.remove(index.clone());
					temp_tweets.insert(index, tw);
					break;
				}
			}

			<TweetsStore<T>>::put(temp_tweets);

			Self::deposit_event(Event::UserLikedATweet(total_likes, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn dislike_a_tweet(origin: OriginFor<T>, tweet_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			let mut temp_tweets = tweets.clone();

			let mut total_likes = 0;
			for (index, tweet) in tweets.iter().enumerate() {
				let mut tw = tweet.clone();
				if tw.id == tweet_id {
					total_likes = tw.likes - 1;
					tw.likes = total_likes;
					let user_liked_tweet_index =
						tw.user_liked_tweet.iter().position(|x| *x == who.clone()).unwrap();
					tw.user_liked_tweet.remove(user_liked_tweet_index);
					temp_tweets.remove(index.clone());
					temp_tweets.insert(index, tw);
					break;
				}
			}

			<TweetsStore<T>>::put(temp_tweets);

			Self::deposit_event(Event::UserLikedATweet(total_likes, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn get_home_time_line(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let user = <UserStore<T>>::get(&who);
			let mut user_home_time_line_tweets: Vec<Tweet<AccountOf<T>>> = vec![];

			let tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			for tweet_id in user.home_time_line.iter() {
				for tweet in tweets.iter() {
					if tweet.id == *tweet_id {
						user_home_time_line_tweets.push(tweet.clone());
						break;
					}
				}
			}
			Self::deposit_event(Event::UserTimeLine(user_home_time_line_tweets, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn get_public_time_line(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let user = <UserStore<T>>::get(&who);
			let mut user_public_time_line_tweets: Vec<Tweet<AccountOf<T>>> = vec![];

			let tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			for tweet_id in user.home_time_line.iter() {
				for tweet in tweets.iter() {
					if tweet.id == *tweet_id {
						user_public_time_line_tweets.push(tweet.clone());
						break;
					}
				}
			}
			Self::deposit_event(Event::UserTimeLine(user_public_time_line_tweets, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn store_tweet(origin: OriginFor<T>, data: Tweet<AccountOf<T>>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			// store the tweet in that persons account
			let mut user = <UserStore<T>>::get(&who);
			user.home_time_line.push(data.id);
			user.public_time_line.push(data.id);

			//put the tweet to people who follow him
			for follower in user.followers.iter() {
				let mut fuser = <UserStore<T>>::get(follower);
				fuser.public_time_line.push(data.id);
				<UserStore<T>>::insert(follower, fuser);
			}

			<UserStore<T>>::insert(&who, user);

			tweets.push(data.clone());

			<TweetsStore<T>>::put(tweets);

			Self::deposit_event(Event::TweetStored(data, who));
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn bookmark_a_tweet(origin: OriginFor<T>, tweet_id: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut user = <UserStore<T>>::get(&who);
			user.bookmark.push(tweet_id.clone());

			<UserStore<T>>::insert(&who, user);

			Self::deposit_event(Event::UserBookmarkedTweet(tweet_id, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn get_bookmarked_tweets(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let user = <UserStore<T>>::get(&who);

			let tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			let mut user_bookmarked_tweets: Vec<Tweet<AccountOf<T>>> = vec![];

			for bookmark_id in user.bookmark.iter() {
				for tweet in tweets.iter() {
					if &tweet.id == bookmark_id {
						user_bookmarked_tweets.push(tweet.clone());
						break;
					}
				}
			}

			Self::deposit_event(Event::UserBookmarkedTweets(user.bookmark, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn search_tweet_by_tag(origin: OriginFor<T>, tag: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let tweets = match <TweetsStore<T>>::get() {
				Some(tws) => tws,
				None => vec![],
			};

			let mut tweets_that_match_tag: Vec<Tweet<AccountOf<T>>> = vec![];

			for tweet in tweets.iter() {
				if String::from_utf8(tweet.tag.clone())
					.unwrap()
					.contains(&String::from_utf8(tag.clone()).unwrap())
				{
					tweets_that_match_tag.push(tweet.clone());
				}
			}

			Self::deposit_event(Event::UserSearchTweetByTag(tweets_that_match_tag, who));
			Ok(())
		}
	}
}
