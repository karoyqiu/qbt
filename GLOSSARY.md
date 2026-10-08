# qbt

qBittorrent 的桌面远程客户端：添加任务、挑选要下载的文件、整理下载结果。

## Language

**任务（torrent）**：
qBittorrent 中的一个种子条目，由 info hash 唯一标识。
_Avoid_: 种子（指 `.torrent` 文件时除外）、下载

**元数据（metadata）**：
任务的文件清单与分片信息。磁力链接添加后需先从网络获取，`.torrent` 文件自带。

**自动选文件（auto-select）**：
按规则为任务设置文件优先级：不小于小文件阈值的文件下载，其余不下载；若全部小于阈值，只下载最大的一个。
_Avoid_: 自动选择、智能选择

**小文件阈值（small file threshold）**：
自动选文件时区分"要下载"与"不下载"的文件大小界限，用户可配置。

**待选文件（pending auto-select）**：
经本 App 新添加、尚未自动选文件的任务。获得元数据后即停止，选完文件再开始下载。
_Avoid_: 已暂停且进度为 0 的任务
