import { Body, fetch, ResponseType } from '@tauri-apps/api/http';
import { assign } from 'radashi';
import { CookieJar } from 'tough-cookie';
import {
  defaultMainData,
  type MainData,
  type ServerState,
  type TorrentContent,
  type TorrentContentPriority,
  type TorrentFilter,
  type TorrentInfo,
} from './qBittorrentTypes';

class QBittorrent {
  private url: string;
  private jar = new CookieJar();
  private loggedIn = false;
  private mainData: MainData = defaultMainData;

  constructor(url: string) {
    this.url = url;
  }

  /**
   * 登录
   * @param username 用户名
   * @param password 密码
   * @returns 是否登录成功
   */
  async login(username: string, password: string) {
    const res = await this.post('auth', 'login', { username, password });
    this.loggedIn = res === 'Ok.';
    return this.loggedIn;
  }

  /** 是否已登录 */
  get hasLoggedIn() {
    return this.loggedIn;
  }

  getGlobalTransferInfo() {
    return this.getObject<ServerState>('transfer', 'info');
  }

  async getMainData() {
    const data = await this.getObject<MainData>('sync', 'maindata', {
      rid: this.mainData.rid.toString(),
    });

    if (this.mainData.rid === 0) {
      console.log(data);
    }

    if (data.full_update) {
      this.mainData = data;
    } else {
      if (data.torrents_removed) {
        for (const t of data.torrents_removed) {
          delete this.mainData.torrents[t];
        }

        delete data.torrents_removed;
      }

      this.mainData = assign(this.mainData, data);
    }

    return this.mainData;
  }

  getTorrentList(params?: {
    filter?: TorrentFilter;
    sort?: keyof TorrentInfo;
    offset?: number;
    limit?: number;
  }) {
    return this.getObject<TorrentInfo[]>('torrents', 'info', params);
  }

  getTorrentContent(hash: string) {
    return this.getObject<TorrentContent[]>('torrents', 'files', { hash });
  }

  add(urls: string | string[]) {
    return this.post('torrents', 'add', {
      urls: Array.isArray(urls) ? urls.join('\n') : urls,
      root_folder: 'true',
    });
  }

  addFile(filename: string) {
    const body = Body.form({
      torrents: {
        file: filename,
        mime: 'application/x-bittorrent',
      },
      paused: 'true',
      root_folder: 'true',
    });

    return this.call('POST', 'torrents', 'add', undefined, body);
  }

  async pause(hashes: string | string[]) {
    await this.post('torrents', 'stop', { hashes: QBittorrent.joinHashes(hashes) });
  }

  async resume(hashes: string | string[]) {
    await this.post('torrents', 'start', { hashes: QBittorrent.joinHashes(hashes) });
  }

  async delete(hashes: string | string[], deleteFiles = true) {
    await this.post('torrents', 'delete', {
      hashes: QBittorrent.joinHashes(hashes),
      deleteFiles,
    });
  }

  /**
   * Set file priority
   * @param hash The hash of the torrent
   * @param indexes File indexes
   * @param priority File priority to set
   */
  async setFilePriority(
    hash: string,
    indexes: number | number[],
    priority: TorrentContentPriority,
  ) {
    const id: string[] = [];

    if (typeof indexes === 'number') {
      id.push(indexes.toString());
    } else {
      for (const index of indexes) {
        id.push(index.toString());
      }
    }

    await this.post('torrents', 'filePrio', {
      hash,
      id: QBittorrent.joinHashes(id),
      priority,
    });
  }

  private async call<T>(
    method: 'GET' | 'POST',
    apiName: string,
    methodName: string,
    query?: Record<string, string | number | boolean>,
    body?: Body,
    responseType?: ResponseType,
  ) {
    const url = new URL(`/api/v2/${apiName}/${methodName}`, this.url).toString();
    const cookie = await this.jar.getCookieString(url);
    const res = await fetch<T>(url, {
      method,
      query,
      headers: { cookie },
      body,
      responseType,
    });

    const header = res.headers['set-cookie'];

    if (header) {
      await this.jar.setCookie(header, url);
    }

    return res.data;
  }

  // private async get(
  //   apiName: string,
  //   methodName: string,
  //   params?: Record<string, string | number | boolean>,
  // ) {
  //   const res = await this.call('GET', apiName, methodName, params);
  //   return res.text();
  // }

  private async getObject<T>(
    apiName: string,
    methodName: string,
    params?: Record<string, string | number | boolean>,
  ) {
    return this.call<T>('GET', apiName, methodName, params);
  }

  private async post(
    apiName: string,
    methodName: string,
    params: Record<string, string | number | boolean | Uint8Array>,
  ) {
    const body = new FormData();

    for (const [key, value] of Object.entries(params)) {
      body.append(key, value.toString());
    }

    return this.call<string>(
      'POST',
      apiName,
      methodName,
      undefined,
      Body.form(body),
      ResponseType.Text,
    );
  }

  private static joinHashes(hashes: string | string[]) {
    return typeof hashes === 'string' ? hashes : hashes.join('|');
  }
}

export default QBittorrent;
