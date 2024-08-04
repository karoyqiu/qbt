const threshold = 1024 as const;
const sizeUnits = ['byte', 'kilobyte', 'megabyte', 'gigabyte', 'terabyte', 'petabyte'] as const;
const sizeFormatters = Object.freeze(
  sizeUnits.map((unit) =>
    Intl.NumberFormat(undefined, {
      style: 'unit',
      unit,
      maximumFractionDigits: 2,
    }),
  ),
);
const speedFormatters = Object.freeze(
  sizeUnits.map((unit) =>
    Intl.NumberFormat(undefined, {
      style: 'unit',
      unit: `${unit}-per-second`,
      maximumFractionDigits: 2,
    }),
  ),
);
const percentFormatter = new Intl.NumberFormat(undefined, {
  style: 'percent',
  minimumFractionDigits: 2,
});

const formatWith = (value: number, formatters: readonly Intl.NumberFormat[]) => {
  let i = 0;
  let n = value;

  while (n > threshold && i < formatters.length) {
    n /= threshold;
    i += 1;
  }

  const formatter = formatters[i];
  return formatter.format(n);
};

export const formatSize = (bytes: number) => formatWith(bytes, sizeFormatters);
export const formatSpeed = (bps: number) => formatWith(bps, speedFormatters);

export const formatPercent = (value: number) => percentFormatter.format(value);
