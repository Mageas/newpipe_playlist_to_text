use rusqlite::{Connection, OpenFlags};

use crate::{NewpipedError, NewpipedResult};

/// Database representation
#[derive(Debug)]
pub struct Database<'a> {
    connection: Connection,
    path: &'a str,
}

impl<'a> Database<'a> {
    /// Connect to the database
    pub fn new(path: &'a str) -> NewpipedResult<Self> {
        Ok(Self {
            connection: Connection::open_with_flags(
                path,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_URI
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .map_err(|e| NewpipedError::DatabaseConnection(e, path.to_owned()))?,
            path,
        })
    }

    /// Query the database
    pub fn query(&self) -> NewpipedResult<Vec<Playlist>> {
        let mut stmt = self
            .connection
            .prepare(
                r#"SELECT p.name, s.url, psj.join_index
    FROM playlist_stream_join as psj
    INNER JOIN playlists as p
        ON psj.playlist_id = p.uid
    INNER JOIN streams as s
        ON psj.stream_id = s.uid
    ORDER BY p.name, psj.join_index
    "#,
            )
            .map_err(|e| NewpipedError::DatabasePrepareStatement(e, self.path.to_owned()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(Row {
                    name: row.get(0)?,
                    url: row.get(1)?,
                })
            })
            .map_err(|e| NewpipedError::DatabaseQuery(e, self.path.to_owned()))?
            .into_iter()
            .map(|row| row.unwrap())
            .collect::<Vec<Row>>();

        let playlists = Self::parse_playlists(rows);

        Ok(playlists)
    }

    /// Parse the database output to a friendly format
    fn parse_playlists(rows: Vec<Row>) -> Vec<Playlist> {
        let mut playlists: Vec<Playlist> = vec![];
        let mut actual_playlist: Playlist = Playlist::new("".to_string());

        for row in rows {
            if row.name != actual_playlist.name {
                playlists.push(actual_playlist);
                actual_playlist = Playlist::new(row.name)
            }
            actual_playlist.push(row.url);
        }
        playlists.remove(0);
        playlists
    }
}

/// Row of a database
#[derive(Debug)]
struct Row {
    name: String,
    url: String,
}

/// Playlist representation
#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub urls: Vec<String>,
}

impl Playlist {
    /// New playlist
    fn new(name: String) -> Self {
        Self { name, urls: vec![] }
    }

    /// Push url to the playlist
    fn push(&mut self, url: String) {
        self.urls.push(url);
    }
}
