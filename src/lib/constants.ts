import type { SelectOption } from './types';

export const audioCodecOptions: SelectOption[] = [
	{ value: 'aac', label: 'AAC' },
	{ value: 'mp3', label: 'MP3' },
	{ value: 'ac3', label: 'AC-3' },
	{ value: 'alac', label: 'ALAC' },
	{ value: 'libopus', label: 'Opus (libopus)' },
	{ value: 'pcm_s16le', label: 'pcm_s16le' },
	{ value: 'libvorbis', label: 'Vorbis (libvorbis)' },
	{ value: 'flac', label: 'FLAC' },
	{ value: 'mp2', label: 'MP2' },
	{ value: 'mp1', label: 'MP1' }
] as const;

export const videoCodecOptions: SelectOption[] = [
	{ value: 'h264', label: 'H.264 (AVC)' },
	{ value: 'vp9', label: 'VP9' },
	{ value: 'vp8', label: 'VP8' },
	{ value: 'av1', label: 'AV1' },
	{ value: 'mpeg4', label: 'MPEG-4 Part 2' },
	{ value: 'prores', label: 'ProRes' },
	{ value: 'libxvid', label: 'Xvid (libxvid)' },
	{ value: 'mjpeg', label: 'MJPEG' },
	{ value: 'h263p', label: 'H.263+' },
	{ value: 'mpeg2video', label: 'MPEG-2' },
	{ value: 'mpeg1video', label: 'MPEG-1' },
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
		audio: ['aac', 'mp3', 'ac3', 'alac', 'opus', 'pcm_s16le'],
		video: ['h264', 'vp9', 'av1']
	},
	mkv: {
		audio: ['aac', 'mp3', 'ac3', 'alac', 'opus', 'pcm_s16le', 'vorbis', 'flac'],
		video: [
			'h264',
			'vp9',
			'av1',
			'mpeg4',
			'prores',
			'libxvid',
			'mjpeg',
			'h263p',
			'mpeg2video',
			'mpeg1video',
			'theora'
		]
	},
	avi: {
		audio: ['mp3', 'ac3', 'pcm_s16le'],
		video: ['mpeg4', 'libxvid', 'mjpeg', 'h263p', 'mpeg2video', 'mpeg1video']
	},
	mov: {
		audio: ['aac', 'alac', 'pcm_s16le'],
		video: ['h264', 'prores', 'mpeg4', 'h263p', 'mpeg2video', 'mpeg1video']
	},
	webm: {
		audio: ['libopus', 'libvorbis'],
		video: ['vp9', 'vp8']
	},
	mpg: {
		audio: ['mp2', 'mp3', 'ac3', 'pcm_s16le'],
		video: ['mpeg1video', 'mpeg2video', 'mpeg4']
	},
	mpeg: {
		audio: ['mp2', 'mp3', 'ac3', 'pcm_s16le'],
		video: ['mpeg1video', 'mpeg2video', 'mpeg4']
	},
	ogv: {
		audio: ['libopus', 'libvorbis'],
		video: ['theora']
	}
};
