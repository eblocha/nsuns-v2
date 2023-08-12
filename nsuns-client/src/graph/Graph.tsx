import { Component, JSX, Show, createMemo, mergeProps } from "solid-js";

export type Point = {
  x: number;
  y: number;
};

type Bounds = {
  minX: number;
  maxX: number;
  minY: number;
  maxY: number;
};

export type GraphStyle = "scatter" | "line";

const getBounds = (data: Point[]): Bounds => {
  const minX = data.length ? data.reduce((minX, point) => Math.min(point.x, minX), Infinity) : 0;
  const maxX = data.length ? data.reduce((maxX, point) => Math.max(point.x, maxX), -Infinity) : 0;

  const minY = data.length ? data.reduce((minY, point) => Math.min(point.y, minY), Infinity) : 0;
  const maxY = data.length ? data.reduce((maxY, point) => Math.max(point.y, maxY), -Infinity) : 0;

  return {
    minX,
    maxX,
    minY,
    maxY,
  };
};

export const Graph: Component<{
  data?: Point[];
  svgProps?: JSX.SvgSVGAttributes<SVGSVGElement>;
  weight?: number;
  width?: string;
  height?: string;
  fillOpacity?: string | number;
}> = (props) => {
  const mergedProps = mergeProps(
    {
      data: [] as Point[],
      weight: 5,
      width: "100%",
      height: "100%",
    },
    props
  );

  const bounds = createMemo(() => getBounds(mergedProps.data));

  const shifted = createMemo(() => {
    const { minX, maxX, minY, maxY } = bounds();
    return mergedProps.data.map((point) => ({
      x: maxX === minX ? 50 : ((point.x - minX) / (maxX - minX)) * 100,
      y: maxY === minY ? 50 : ((minY - point.y + (maxY - minY)) / (maxY - minY)) * 100,
    }));
  });

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      {...props.svgProps}
      preserveAspectRatio={mergedProps.data.length <= 1 ? undefined : "none"}
      height={mergedProps.height}
      width={mergedProps.width}
      viewBox={`${-mergedProps.weight / 2} ${-mergedProps.weight / 2} ${100 + mergedProps.weight} ${
        100 + mergedProps.weight
      }`}
    >
      <Show
        when={mergedProps.data.length > 1}
        fallback={
          <Show when={mergedProps.data.length}>
            <SinglePoint
              point={{ x: 50, y: 50 }}
              weight={mergedProps.weight}
            />
          </Show>
        }
      >
        <Line
          data={shifted()}
          weight={mergedProps.weight}
          fillOpacity={props.fillOpacity}
        />
      </Show>
    </svg>
  );
};

const SinglePoint: Component<{ point: Point; weight: number }> = (props) => {
  return (
    <circle
      cx={props.point.x}
      cy={props.point.y}
      r={props.weight}
      fill="currentColor"
    />
  );
};

const Line: Component<{
  data: Point[];
  weight: number;
  fillOpacity?: string | number;
}> = (props) => {
  const instructions = createMemo(() => {
    let path = "";

    props.data.forEach((point, index) => {
      if (index === 0) {
        path += `M ${point.x} ${point.y}`;
        return;
      }

      path += ` L ${point.x} ${point.y}`;
    });

    return path;
  });

  const fillInstructions = createMemo(() => {
    let path = instructions();

    const maxX = props.data.reduce((maxX, point) => Math.max(point.x, maxX), -Infinity);
    const minX = props.data.reduce((x, point) => Math.min(point.x, x), Infinity);
    const maxY = props.data.reduce((maxY, point) => Math.max(point.y, maxY), -Infinity);

    path += ` L ${maxX} ${maxY} L ${minX} ${maxY}`;
    return path;
  });

  return (
    <>
      <path
        d={instructions() || undefined}
        stroke="currentColor"
        fill="none"
        stroke-linejoin="round"
        stroke-width={props.weight}
        vector-effect="non-scaling-stroke"
      />
      <Show when={props.fillOpacity}>
        <path
          d={fillInstructions()}
          stroke="none"
          fill="currentColor"
          fill-opacity={props.fillOpacity}
          vector-effect="non-scaling-stroke"
        />
      </Show>
    </>
  );
};
