#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]
struct TvChannel {
    name: String,
    channel_type: ChannelType,
}

fn main() {
    let channels: [TvChannel; 4] = [
        TvChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TvChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TvChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TvChannel {
            name: String::from("RustTv"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    let good_channels: Vec<String> = channels
        .iter()
        .filter(|tv_channel: &&TvChannel| {
            tv_channel.channel_type == ChannelType::ProgrammingTutorials
        })
        .map(|tv_channel: &TvChannel| {
            tv_channel.name.clone()
        })
        .collect();
    println!("{:?}", good_channels);

    let good_channel: Option<&TvChannel> = channels
        .iter()
        .find(|tv_channel: &&TvChannel| {
            tv_channel.channel_type == ChannelType::ProgrammingTutorials
        });

    match good_channel {
        Some(tv_channel) => {
            println!("Great choice to watch {:?}", tv_channel);
        }
        None => {
            println!("There was no Rust programming on the TV (literally and metaphorically)");
        }
    }
}
