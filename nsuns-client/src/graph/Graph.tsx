import {
  Component,
  For,
  JSX,
  Match,
  Show,
  Switch,
  createMemo,
  mergeProps,
} from "solid-js";

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
  const minX = data.length
    ? data.reduce((minX, point) => Math.min(point.x, minX), Infinity)
    : 0;
  const maxX = data.length
    ? data.reduce((maxX, point) => Math.max(point.x, maxX), -Infinity)
    : 1;

  const minY = data.length
    ? data.reduce((minY, point) => Math.min(point.y, minY), Infinity)
    : 0;
  const maxY = data.length
    ? data.reduce((maxY, point) => Math.max(point.y, maxY), -Infinity)
    : 1;

  return {
    minX,
    maxX,
    minY,
    maxY,
  };
};

export const Graph: Component<{
  data?: Point[];
  style?: GraphStyle;
  svgProps?: JSX.SvgSVGAttributes<SVGSVGElement>;
  weight?: number;
  width?: string;
  height?: string;
  fillOpacity?: string | number;
}> = (props) => {
  const mergedProps = mergeProps(
    {
      data: [] as Point[],
      style: "scatter" as GraphStyle,
      weight: 5,
      width: "100%",
      height: "100%",
    },
    props
  );

  const bounds = createMemo(() => getBounds(mergedProps.data));

  const shifted = createMemo(() => {
    const { minX, maxX, minY, maxY } = bounds();
    console.log(bounds())
    return mergedProps.data.map((point) => ({
      x: maxX === minX ? 50 : ((point.x - minX) / (maxX - minX)) * 100,
      y: maxY === minY ? 50 : ((minY - point.y + (maxY - minY)) / (maxY - minY)) * 100,
    }));
  });

  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      {...props.svgProps}
      height={mergedProps.height}
      width={mergedProps.width}
      viewBox={`${-mergedProps.weight / 2} ${-mergedProps.weight / 2} ${
        100 + mergedProps.weight
      } ${100 + mergedProps.weight}`}
    >
      <Switch>
        <Match
          when={mergedProps.style === "scatter" || mergedProps.data.length <= 1}
        >
          <Scatter data={shifted()} weight={mergedProps.weight} />
        </Match>
        <Match when={mergedProps.style === "line"}>
          <Line
            data={shifted()}
            weight={mergedProps.weight}
            fillOpacity={props.fillOpacity}
          />
        </Match>
      </Switch>
    </svg>
  );
};

const Scatter: Component<{ data: Point[]; weight: number }> = (props) => {
  return (
    <For each={props.data}>
      {(point) => (
        <circle
          cx={point.x}
          cy={point.y}
          r={props.weight / 2}
          fill="currentColor"
        />
      )}
    </For>
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

    const maxX = props.data.reduce(
      (maxX, point) => Math.max(point.x, maxX),
      -Infinity
    );
    const maxY = props.data.reduce(
      (maxY, point) => Math.max(point.y, maxY),
      -Infinity
    );

    path += ` L ${maxX} ${maxY}`;
    return path;
  });

  const firstPoint = () => props.data[0];
  const lastPoint = () => props.data[props.data.length - 1];

  return (
    <>
      <Show when={firstPoint()}>
        <circle
          cx={firstPoint().x}
          cy={firstPoint().y}
          r={props.weight / 2}
          fill="currentColor"
        />
      </Show>

      <path
        d={instructions() || undefined}
        stroke="currentColor"
        fill="none"
        stroke-linejoin="round"
        stroke-width={props.weight}
      />
      <Show when={props.fillOpacity}>
        <path
          d={fillInstructions()}
          stroke="none"
          fill="currentColor"
          fill-opacity={props.fillOpacity}
        />
      </Show>
      <Show when={lastPoint()}>
        <circle
          cx={lastPoint().x}
          cy={lastPoint().y}
          r={props.weight / 2}
          fill="currentColor"
        />
      </Show>
    </>
  );
};
