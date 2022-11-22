export interface Song {
    // The song's metadata.
    Metadata: SongMetadata;
    // The ID of the song.
    ID: string;
}

// Represents metadata about a song like: name, artist, albumn, etc.
export interface SongMetadata {
    // The name of the song.
    Name: string;
}