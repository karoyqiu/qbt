# 用 qBittorrent tag 标记待选文件任务

新任务添加时带 `stopCondition=MetadataReceived` 和 tag `qbt:auto-select`，App 每次刷新扫描带该 tag 且已有元数据的任务，自动选文件、开始下载、移除 tag。标记存在 qBittorrent 里，App 关闭或换机器都不会丢失，下次打开即补处理。

## Considered Options

- **按"已暂停且进度为 0"判断**：无需标记，但会把用户刚添加就手动停止的任务重新选文件并启动。
- **按上述条件判断，并在本地记录用户手动停止过的任务**：需要额外的本地状态，且与 qBittorrent 的真实状态可能不一致。
- **原方案：前端内存跟踪 `metaDL` 状态**：任务在选文件前已开始下载；App 未运行期间完成元数据下载的任务永远不会被处理。

## Consequences

- tag `qbt:auto-select` 在 qBittorrent WebUI 等其他客户端中可见，选完文件后移除。
- 本功能上线前已存在的任务没有 tag，不会被自动处理。
