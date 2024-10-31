use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space1},
    combinator::map_res,
    IResult,
};

/// Parses a string input to extract an episode number and title.
///
/// This function takes a string input and attempts to parse it to extract an episode number and title. The input should be in the format of either "Episode <number>: <title>" or "E<number>: <title>". The function will return a tuple containing the episode number as an i32 and the title as a string reference.
///
/// # Arguments
///
/// * `input` - A string slice that contains the episode information in the format "Episode <number>: <title>" or "E<number>: <title>"
///
/// # Returns
///
/// A Result containing a tuple with the episode number as an i32 and the title as a string reference. If successful, it returns Ok((episode_number, episode_title)). If parsing fails, it returns an error.
///
/// # Examples
///
///
pub fn parse_episode(input: &str) -> IResult<&str, (i32, &str)> {
    // NOTE: Try to find either "Episode" or "E" followed by the number and title
    alt((parse_new_episode, reparse_episode))(input)
}

/// Parses a full episode string and returns a tuple containing the episode number and title.
///
/// # Arguments
///
/// * `input` - A string slice that represents the full episode string to be parsed.
///
/// # Returns
///
/// Returns a `Result` with a tuple containing the episode number as an `i32` and the title as a `&str`.
///
/// # Errors
///
/// Returns an error if parsing fails or if the input string does not match the expected format.
///
/// # Examples
///
///
pub fn parse_new_episode(input: &str) -> IResult<&str, (i32, &str)> {
    let (input, _) = take_until("Episode")(input)?;
    let (input, _) = tag("Episode")(input)?;
    let (input, _) = space1(input)?;
    let (input, episode_number) = map_res(digit1, str::parse::<i32>)(input)?;
    let (input, title) = take_until(".")(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((input, (episode_number, title.trim())))
}

/// Parses a string representing an episode into its episode number and title.
///
/// # Arguments
///
/// * `input` - A string slice that contains the episode information in the format "E<number>.<title>"
///
/// # Returns
///
/// Returns a tuple containing the remaining input string and a tuple with the episode number and title.
///
/// # Errors
///
/// Returns an error if the input string does not match the expected format.
///
/// # Examples
///
///
pub fn reparse_episode(input: &str) -> IResult<&str, (i32, &str)> {
    let (input, _) = take_until("E")(input)?;
    let (input, _) = tag("E")(input)?;
    let (input, episode_number) = map_res(digit1, str::parse::<i32>)(input)?;
    let (input, title) = take_until(".")(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((input, (episode_number, title.trim())))
}
