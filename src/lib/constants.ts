import type { SelectOption } from '@melt-ui/svelte';
import type { CodecOption } from './types';

export const noneOption: SelectOption = { value: undefined, label: 'None' };

export const audioCodecOptions: SelectOption<string>[] = [
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

export const videoCodecOptions: SelectOption<string>[] = [
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

export const extensionOptions: SelectOption<string>[] = [
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
		audio: ['aac', 'mp3', 'ac3', 'alac', 'libopus', 'pcm_s16le'],
		video: ['h264', 'vp9', 'av1']
	},
	mkv: {
		audio: ['aac', 'mp3', 'ac3', 'alac', 'libopus', 'pcm_s16le', 'libvorbis', 'flac'],
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

export const videoCodecConfig: Record<string, CodecOption[]> = {
	h264: [
		{
			name: 'Profile',
			ffmpegKey: 'profile',
			type: 'select',
			options: ['baseline', 'main', 'high'],
			defaultValue: 'main',
			description: 'Specifies the H.264 profile to use.'
		},
		{
			name: 'Preset',
			ffmpegKey: 'preset',
			type: 'select',
			options: [
				'ultrafast',
				'superfast',
				'veryfast',
				'faster',
				'fast',
				'medium',
				'slow',
				'slower',
				'veryslow'
			],
			defaultValue: 'medium',
			description: 'Preset to balance encoding speed and compression efficiency.'
		},
		{
			name: 'CRF',
			ffmpegKey: 'crf',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 51 },
			defaultValue: 23,
			description: 'Constant Rate Factor (CRF) controls the quality of the video.'
		}
	],
	vp9: [
		{
			name: 'Quality',
			ffmpegKey: 'crf',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 63 },
			defaultValue: 31,
			description: 'Controls the quality of VP9 encoding.'
		},
		{
			name: 'Tile Columns',
			ffmpegKey: 'tile-columns',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 6 },
			defaultValue: 0,
			description: 'Number of tile columns for parallel encoding.'
		}
	],
	vp8: [
		{
			name: 'Deadline',
			ffmpegKey: 'deadline',
			type: 'select',
			options: ['best', 'good', 'realtime'],
			defaultValue: 'good',
			description: 'Controls the encoding speed vs quality tradeoff.'
		}
	],
	av1: [
		{
			name: 'CQ Level',
			ffmpegKey: 'cq-level',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 63 },
			defaultValue: 30,
			description: 'Controls the quality level for AV1 encoding.'
		},
		{
			name: 'CPU-Used',
			ffmpegKey: 'cpu-used',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 8 },
			defaultValue: 4,
			description: 'Controls the encoding speed for AV1.'
		}
	],
	mpeg4: [
		{
			name: 'GOP Size',
			ffmpegKey: 'g',
			type: 'input',
			inputType: 'number',
			validation: { min: 0 },
			defaultValue: 12,
			description: 'Sets the maximum GOP size.'
		}
	],
	prores: [
		{
			name: 'Profile',
			ffmpegKey: 'profile:v',
			type: 'select',
			options: ['proxy', 'lt', 'standard', 'hq', '4444', '4444xq'],
			defaultValue: 'standard',
			description: 'Specifies the ProRes profile.'
		}
	],
	libxvid: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:v',
			type: 'input',
			inputType: 'number',
			validation: { min: 0 },
			defaultValue: 1000,
			description: 'Specifies the target bitrate for Xvid encoding.'
		}
	],
	mjpeg: [
		{
			name: 'Quality',
			ffmpegKey: 'q:v',
			type: 'input',
			inputType: 'number',
			validation: { min: 2, max: 31 },
			defaultValue: 7,
			description: 'Controls the quality of MJPEG encoding.'
		}
	],
	h263p: [
		{
			name: 'GOP Size',
			ffmpegKey: 'g',
			type: 'input',
			inputType: 'number',
			validation: { min: 0 },
			defaultValue: 12,
			description: 'Sets the maximum GOP size for H.263+.'
		}
	],
	mpeg2video: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:v',
			type: 'input',
			inputType: 'number',
			validation: { min: 0 },
			defaultValue: 2000,
			description: 'Specifies the target bitrate for MPEG-2 encoding.'
		}
	],
	mpeg1video: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:v',
			type: 'input',
			inputType: 'number',
			validation: { min: 0 },
			defaultValue: 1500,
			description: 'Specifies the target bitrate for MPEG-1 encoding.'
		}
	],
	theora: [
		{
			name: 'Quality',
			ffmpegKey: 'q:v',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 10 },
			defaultValue: 5,
			description: 'Controls the quality of Theora encoding.'
		}
	]
};

export const audioCodecConfig: Record<string, CodecOption[]> = {
	aac: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:a',
			type: 'input',
			inputType: 'number',
			validation: { min: 8, max: 512 },
			defaultValue: 128,
			description: 'Specifies the target bitrate for AAC encoding (in kbps).'
		},
		{
			name: 'Profile',
			ffmpegKey: 'profile:a',
			type: 'select',
			options: ['aac_low', 'aac_he', 'aac_he_v2'],
			defaultValue: 'aac_low',
			description: 'Specifies the AAC profile.'
		}
	],
	mp3: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:a',
			type: 'input',
			inputType: 'number',
			validation: { min: 8, max: 320 },
			defaultValue: 192,
			description: 'Specifies the target bitrate for MP3 encoding (in kbps).'
		}
	],
	ac3: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:a',
			type: 'input',
			inputType: 'number',
			validation: { min: 32, max: 640 },
			defaultValue: 384,
			description: 'Specifies the target bitrate for AC-3 encoding (in kbps).'
		}
	],
	alac: [
		{
			name: 'Compression Level',
			ffmpegKey: 'compression_level',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 12 },
			defaultValue: 5,
			description: 'Controls the compression level for ALAC encoding.'
		}
	],
	libopus: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:a',
			type: 'input',
			inputType: 'number',
			validation: { min: 6, max: 510 },
			defaultValue: 128,
			description: 'Specifies the target bitrate for Opus encoding (in kbps).'
		},
		{
			name: 'Complexity',
			ffmpegKey: 'compression_level',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 10 },
			defaultValue: 10,
			description: 'Controls the encoding complexity for Opus.'
		}
	],
	pcm_s16le: [
		{
			name: 'Sample Rate',
			ffmpegKey: 'ar',
			type: 'input',
			inputType: 'number',
			validation: { min: 8000, max: 192000 },
			defaultValue: 44100,
			description: 'Specifies the sample rate for PCM encoding.'
		}
	],
	libvorbis: [
		{
			name: 'Quality',
			ffmpegKey: 'q:a',
			type: 'input',
			inputType: 'number',
			validation: { min: -1, max: 10 },
			defaultValue: 3,
			description: 'Controls the quality for Vorbis encoding.'
		}
	],
	flac: [
		{
			name: 'Compression Level',
			ffmpegKey: 'compression_level',
			type: 'input',
			inputType: 'number',
			validation: { min: 0, max: 12 },
			defaultValue: 5,
			description: 'Specifies the compression level for FLAC encoding.'
		}
	],
	mp2: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:a',
			type: 'input',
			inputType: 'number',
			validation: { min: 32, max: 384 },
			defaultValue: 192,
			description: 'Specifies the target bitrate for MP2 encoding (in kbps).'
		}
	],
	mp1: [
		{
			name: 'Bitrate',
			ffmpegKey: 'b:a',
			type: 'input',
			inputType: 'number',
			validation: { min: 32, max: 384 },
			defaultValue: 192,
			description: 'Specifies the target bitrate for MP1 encoding (in kbps).'
		}
	]
};
