# NewPipe Playlist To Text

Export your NewPipe playlists to text files.

## **How to use**

``` text
Usage: newpipe_playlist_to_text [OPTIONS] --database <DATABASE>

Options:
  -d, --database <DATABASE>  Path of the database file
  -o, --output <OUTPUT>      Path of the output directory
      --overwrite            Overwirite the existing playlists
  -h, --help                 Print help information
  -V, --version              Print version information
```

## **Install instructions**

Clone the repository:
```
git clone https://gitea.heartnerds.org/Mageas/newpipe_playlist_to_text
```

Move into the project directory:
```
cd newpipe_playlist_to_text
```

Install the project with cargo:
```
cargo install --path=.
```
