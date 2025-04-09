import type {
  TreeTableCheckboxSelectionKeyType,
  TreeTableExpandedKeysType,
} from 'primereact/treetable';

import type { TorrentNode } from '../ui/TorrentDialog';
import type { TorrentContent } from './bindings';
import { TorrentContentPriority } from './qBittorrentTypes';

const findChild = (parent: TorrentNode, childName: string) => {
  if (parent.children) {
    for (const child of parent.children) {
      if (child.data.name === childName) {
        return child;
      }
    }
  }

  return null;
};

const isSelected = (child: TorrentNode) =>
  child.data.priority !== TorrentContentPriority.DO_NOT_DOWNLOAD;

const finalize = (
  parent: TorrentNode,
  selected: Record<string, TreeTableCheckboxSelectionKeyType>,
  expanded: TreeTableExpandedKeysType,
) => {
  if (!parent.children || parent.children.length === 0) {
    return { checked: !!selected[parent.key!]?.checked };
  }

  let checkedCount = 0;
  let downloaded = 0;
  parent.data.size = 0;

  for (const child of parent.children) {
    const checked = finalize(child, selected, expanded);

    if (checked.checked || checked.partialChecked) {
      checkedCount++;
      downloaded += child.data.size * child.data.progress;
      parent.data.size += child.data.size;
    }
  }

  parent.data.progress = parent.data.size > 0 ? downloaded / parent.data.size : 0;

  const checked: TreeTableCheckboxSelectionKeyType =
    checkedCount === 0
      ? { checked: false }
      : checkedCount === parent.children.length
        ? { checked: true }
        : { partialChecked: true };
  selected[parent.key!] = checked;

  if (checked.checked || checked.partialChecked) {
    expanded[parent.key!] = true;
  }

  return checked;
};

const makeTree = (content: TorrentContent[]) => {
  const root: TorrentNode = {
    key: 'root',
    data: {
      index: -1,
      name: '',
      fullPath: '',
      size: 0,
      priority: TorrentContentPriority.DO_NOT_DOWNLOAD,
      progress: 0,
      //is_seed: false,
      piece_range: [],
      availability: 0,
    },
    children: [],
  };

  const selected: Record<string, TreeTableCheckboxSelectionKeyType> = {};

  for (const c of content) {
    if (c.name.includes('_____padding_file_')) {
      continue;
    }

    const parts = c.name.split('/');
    let parent = root;

    // 查找已有的部分
    while (parts.length > 1) {
      const child = findChild(parent, parts[0]);

      if (child) {
        parent = child;
        parts.shift();
      } else {
        break;
      }
    }

    // 创建没有的部分
    while (parts.length > 0) {
      const child: TorrentNode = {
        key: crypto.randomUUID(),
        data: { ...c, fullPath: c.name },
      };

      child.data.name = parts.shift()!;

      if (parts.length > 0) {
        // 这是个文件夹
        child.data.index = -1;
        child.data.priority = TorrentContentPriority.DO_NOT_DOWNLOAD;
      } else if (isSelected(child)) {
        selected[child.key! as string] = { checked: true };
      }

      if (parent.children) {
        parent.children.push(child);
      } else {
        parent.children = [child];
      }

      parent = child;
    }
  }

  const expanded: TreeTableExpandedKeysType = {};
  finalize(root, selected, expanded);

  for (const [key, value] of Object.entries(selected)) {
    if (key === 'root' || (!value.checked && !value.partialChecked)) {
      delete selected[key];
    }
  }

  return { nodes: root.children!, selected, expanded };
};

export default makeTree;
