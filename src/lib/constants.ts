import type { SelectOption } from './types';

export const audioCodecOptions: SelectOption[] = [
	{ value: 'aac', label: 'AAC' },
	{ value: 'mp3', label: 'MP3' },
	{ value: 'ac3', label: 'AC-3' },
	{ value: 'alac', label: 'ALAC' },
	{ value: 'opus', label: 'Opus' },
	{ value: 'pcm', label: 'PCM' },
	{ value: 'vorbis', label: 'Vorbis' },
	{ value: 'dts', label: 'DTS' },
	{ value: 'flac', label: 'FLAC' },
	{ value: 'mp2', label: 'MP2' },
	{ value: 'mp1', label: 'MP1' }
] as const;

export const videoCodecOptions: SelectOption[] = [
	{ value: 'h264', label: 'H.264 (AVC)' },
	{ value: 'h265', label: 'H.265 (HEVC)' },
	{ value: 'vp9', label: 'VP9' },
	{ value: 'av1', label: 'AV1' },
	{ value: 'mpeg4', label: 'MPEG-4 Part 2' },
	{ value: 'prores', label: 'ProRes' },
	{ value: 'xvid', label: 'Xvid' },
	{ value: 'divx', label: 'DivX' },
	{ value: 'mjpeg', label: 'MJPEG' },
	{ value: 'h263', label: 'H.263' },
	{ value: 'mpeg2', label: 'MPEG-2' },
	{ value: 'mpeg1', label: 'MPEG-1' },
	{ value: 'theora', label: 'Theora' }
] as const;

export const extensionOptions: SelectOption[] = [
	{ value: 'mp4', label: 'MP4' },
	{ value: 'mkv', label: 'MKV' },
	{ value: 'avi', label: 'AVI' },
	{ value: 'mov', label: 'MOV' },
	{ value: 'webm', label: 'WEBM' },
	{ value: 'mpg', label: 'MPG' },
	{ value: 'mpeg', label: 'MPEG' },
	{ value: 'ogv', label: 'OGV' }
] as const;

export const formatCodecs: Readonly<
	Record<string, { audio: ReadonlyArray<string>; video: ReadonlyArray<string> }>
> = {
	mp4: {
		audio: ['aac', 'mp3', 'ac3', 'alac', 'opus', 'pcm'],
		video: ['h264', 'h265', 'vp9', 'av1']
	},
	mkv: {
		audio: ['aac', 'mp3', 'ac3', 'alac', 'opus', 'pcm', 'vorbis', 'dts', 'flac'],
		video: [
			'h264',
			'h265',
			'vp9',
			'av1',
			'mpeg4',
			'prores',
			'xvid',
			'divx',
			'mjpeg',
			'h263',
			'mpeg2',
			'mpeg1',
			'theora'
		]
	},
	avi: {
		audio: ['mp3', 'ac3', 'pcm'],
		video: ['mpeg4', 'xvid', 'divx', 'mjpeg', 'h263', 'mpeg2', 'mpeg1']
	},
	mov: {
		audio: ['aac', 'alac', 'pcm'],
		video: ['h264', 'prores', 'mpeg4', 'h263', 'mpeg2', 'mpeg1']
	},
	webm: {
		audio: ['opus', 'vorbis'],
		video: ['vp9', 'vp8']
	},
	mpg: {
		audio: ['mp2', 'mp3', 'ac3', 'pcm'],
		video: ['mpeg1', 'mpeg2', 'mpeg4']
	},
	mpeg: {
		audio: ['mp2', 'mp3', 'ac3', 'pcm'],
		video: ['mpeg1', 'mpeg2', 'mpeg4']
	},
	ogv: {
		audio: ['opus', 'vorbis'],
		video: ['theora']
	}
};
