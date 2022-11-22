import { writable } from 'svelte/store';
import type { Song } from './schemas'

export const Songs = writable(new Array<Song>())