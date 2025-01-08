// @ts-nocheck

// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
/**
 * 添加文件
 */
async addFiles(paths: string[]) : Promise<null> {
    return await TAURI_INVOKE("add_files", { paths });
},
/**
 * 添加链接
 */
async addUrls(urls: string) : Promise<null> {
    return await TAURI_INVOKE("add_urls", { urls });
},
/**
 * 删除
 */
async delete(hashes: string[]) : Promise<null> {
    return await TAURI_INVOKE("delete", { hashes });
},
/**
 * 获取主要数据
 */
async getMainData() : Promise<MainData> {
    return await TAURI_INVOKE("get_main_data");
},
/**
 * 获取种子内容
 */
async getTorrentContents(hash: string) : Promise<TorrentContent[]> {
    return await TAURI_INVOKE("get_torrent_contents", { hash });
},
/**
 * 获取视频信息
 */
async getVideoInfo(name: string) : Promise<VideoInfo | null> {
    return await TAURI_INVOKE("get_video_info", { name });
},
/**
 * 之前是否下载过
 */
async hasBeenDownloaded(name: string) : Promise<number | null> {
    return await TAURI_INVOKE("has_been_downloaded", { name });
},
/**
 * 设置 URL
 */
async initialize(url: string, proxy: string | null) : Promise<null> {
    return await TAURI_INVOKE("initialize", { url, proxy });
},
/**
 * 登录
 */
async login(username: string, password: string) : Promise<boolean> {
    return await TAURI_INVOKE("login", { username, password });
},
/**
 * 标记为已下载
 */
async markAsDownloaded(name: string, downloadedAt: number) : Promise<null> {
    return await TAURI_INVOKE("mark_as_downloaded", { name, downloadedAt });
},
/**
 * 重新校验
 */
async recheck(hashes: string[]) : Promise<null> {
    return await TAURI_INVOKE("recheck", { hashes });
},
/**
 * 设置文件优先级
 */
async setFilePriority(hash: string, indexes: number[], priority: number) : Promise<null> {
    return await TAURI_INVOKE("set_file_priority", { hash, indexes, priority });
},
/**
 * 开始
 */
async start(hashes: string[]) : Promise<null> {
    return await TAURI_INVOKE("start", { hashes });
},
/**
 * 停止
 */
async stop(hashes: string[]) : Promise<null> {
    return await TAURI_INVOKE("stop", { hashes });
}
}

/** user-defined events **/



/** user-defined constants **/



/** user-defined types **/

export type Actress = { name: string; photo: string | null }
export type ConnectionStatus = "connected" | "firewalled" | "disconnected"
export type MainData = { 
/**
 * Whether the response contains all the data or partial data
 */
full_update?: boolean; 
/**
 * Response ID
 */
rid: number; 
/**
 * Global transfer info
 */
server_state?: ServerState | null; 
/**
 * Property: torrent hash, value: same as torrent list
 */
torrents?: { [key in string]: TorrentInfo } | null; 
/**
 * List of hashes of torrents removed since last request
 */
torrents_removed?: string[] | null }
export type ServerState = { alltime_dl?: number | null; alltime_ul?: number | null; average_time_queue?: number | null; 
/**
 * Connection status
 */
connection_status?: ConnectionStatus | null; 
/**
 * DHT nodes connected to
 */
dht_nodes?: number | null; 
/**
 * Data downloaded this session (bytes)
 */
dl_info_data?: number | null; 
/**
 * Global download rate (bytes/s)
 */
dl_info_speed?: number | null; 
/**
 * Download rate limit (bytes/s)
 */
dl_rate_limit?: number | null; free_space_on_disk?: number | null; global_ratio?: string | null; queued_io_jobs?: number | null; 
/**
 * True if torrent queueing is enabled
 */
queueing?: boolean | null; read_cache_hits?: string | null; read_cache_overload?: string | null; 
/**
 * Transfer list refresh interval (milliseconds)
 */
refresh_interval?: number | null; total_buffers_size?: number | null; total_peer_connections?: number | null; total_queued_size?: number | null; total_wasted_session?: number | null; 
/**
 * Data uploaded this session (bytes)
 */
up_info_data?: number | null; 
/**
 * Global upload rate (bytes/s)
 */
up_info_speed?: number | null; 
/**
 * Upload rate limit (bytes/s)
 */
up_rate_limit?: number | null; 
/**
 * True if alternative speed limits are enabled
 */
use_alt_speed_limits?: boolean | null; use_subcategories?: boolean | null; write_cache_overload?: string | null }
export type TorrentContent = { 
/**
 * File index
 */
index: number; 
/**
 * File name (including relative path)
 */
name: string; 
/**
 * File size (bytes)
 */
size: number; 
/**
 * File progress (percentage/100)
 */
progress: number; 
/**
 * File priority
 */
priority: number; 
/**
 * True if file is seeding/complete
 * The first number is the starting piece index and the second number is the ending piece index (inclusive)
 */
piece_range: number[]; 
/**
 * Percentage of file pieces currently available
 */
availability: number }
export type TorrentInfo = { 
/**
 * Time (Unix Epoch) when the torrent was added to the client
 */
added_on?: number | null; 
/**
 * Amount of data left to download (bytes)
 */
amount_left?: number | null; 
/**
 * Whether this torrent is managed by Automatic Torrent Management
 */
auto_tmm?: boolean | null; 
/**
 * Percentage of file pieces currently available
 */
availability?: number | null; 
/**
 * Category of the torrent
 */
category?: string | null; 
/**
 * Amount of transfer data completed (bytes)
 */
completed?: number | null; 
/**
 * Time (Unix Epoch) when the torrent completed
 */
completion_on?: number | null; 
/**
 * Torrent download speed limit (bytes/s). -1 if unlimited.
 */
dl_limit?: number | null; 
/**
 * Torrent download speed (bytes/s)
 */
dlspeed?: number | null; 
/**
 * Amount of data downloaded
 */
downloaded?: number | null; 
/**
 * Amount of data downloaded this session
 */
downloaded_session?: number | null; 
/**
 * Torrent ETA (seconds)
 */
eta?: number | null; 
/**
 * True if first last piece are prioritized
 */
f_l_piece_prio?: boolean | null; 
/**
 * True if force start is enabled for this torrent
 */
force_start?: boolean | null; 
/**
 * Torrent hash
 */
infohash_v1?: string | null; infohash_v2?: string | null; 
/**
 * Last time (Unix Epoch) when a chunk was downloaded/uploaded
 */
last_activity?: number | null; 
/**
 * Magnet URI corresponding to this torrent
 */
magnet_uri?: string | null; 
/**
 * Maximum share ratio until torrent is stopped from seeding/uploading
 */
max_ratio?: number | null; 
/**
 * Maximum seeding time (seconds) until torrent is stopped from seeding
 */
max_seeding_time?: number | null; 
/**
 * Torrent name
 */
name?: string | null; 
/**
 * Number of seeds in the swarm
 */
num_complete?: number | null; 
/**
 * Number of leechers in the swarm
 */
num_incomplete?: number | null; 
/**
 * Number of leechers connected to
 */
num_leechs?: number | null; 
/**
 * Number of seeds connected to
 */
num_seeds?: number | null; 
/**
 * Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
 */
priority?: number | null; 
/**
 * Torrent progress (percentage/100)
 */
progress?: number | null; 
/**
 * Torrent share ratio. Max ratio value: 9999.
 */
ratio?: number | null; 
/**
 * TODO (what is different from max_ratio?)
 */
ratio_limit?: number | null; 
/**
 * Path where this torrent's data is stored
 */
save_path?: string | null; 
/**
 * TODO (what is different from max_seeding_time?)
 */
seeding_time_limit?: number | null; 
/**
 * Time (Unix Epoch) when this torrent was last seen complete
 */
seen_complete?: number | null; 
/**
 * True if sequential download is enabled
 */
seq_dl?: boolean | null; 
/**
 * Total size (bytes) of files selected for download
 */
size?: number | null; 
/**
 * Torrent state
 */
state?: TorrentState | null; 
/**
 * True if super seeding is enabled
 */
super_seeding?: boolean | null; 
/**
 * Comma-concatenated tag list of the torrent
 */
tags?: string | null; 
/**
 * Total active time (seconds)
 */
time_active?: number | null; 
/**
 * Total size (bytes) of all file in this torrent (including unselected ones)
 */
total_size?: number | null; 
/**
 * The first tracker with working status. Returns empty string if no tracker is working.
 */
tracker?: string | null; 
/**
 * Torrent upload speed limit (bytes/s). -1 if unlimited.
 */
up_limit?: number | null; 
/**
 * Amount of data uploaded
 */
uploaded?: number | null; 
/**
 * Amount of data uploaded this session
 */
uploaded_session?: number | null; 
/**
 * Torrent upload speed (bytes/s)
 */
upspeed?: number | null }
export type TorrentState = "error" | "missingFiles" | "uploading" | "stoppedUP" | "queuedUP" | "stalledUP" | "checkingUP" | "forcedUP" | "allocating" | "downloading" | "metaDL" | "stoppedDL" | "queuedDL" | "stalledDL" | "checkingDL" | "forcedDL" | "checkingResumeData" | "moving" | "unknown"
export type TranslatedText = { text: string; translated: string | null }
/**
 * 视频信息
 */
export type VideoInfo = { 
/**
 * 番号
 */
code: string; 
/**
 * 标题
 */
title: TranslatedText; 
/**
 * 海报
 */
poster: string | null; 
/**
 * 封面
 */
cover: string | null; 
/**
 * 简介
 */
outline: TranslatedText | null; 
/**
 * 演员列表
 */
actresses: Actress[] | null; 
/**
 * 标签列表
 */
tags: string[] | null; 
/**
 * 系列
 */
series: string | null; 
/**
 * 片商
 */
studio: string | null; 
/**
 * 发行商
 */
publisher: string | null; 
/**
 * 导演
 */
director: string | null; 
/**
 * 时长（秒）
 */
duration: number | null; 
/**
 * 发布日期（Unix epoch）
 */
release_date: number | null; 
/**
 * 额外的插图
 */
extra_fanart: string[] | null }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
