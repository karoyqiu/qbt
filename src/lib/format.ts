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
const percentFormatter = new Intl.NumberFormat(undefined, {
  style: 'percent',
  minimumFractionDigits: 2,
});

export const formatSize = (bytes: number) => {
  let i = 0;
  let n = bytes;

  while (n > threshold && i < sizeFormatters.length) {
    n /= threshold;
    i += 1;
  }

  const formatter = sizeFormatters[i];
  return formatter.format(n);
};

export const formatPercent = (value: number) => percentFormatter.format(value);
