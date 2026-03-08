import { useId } from "react";

export type BlobProps = {
  colorA: string;
  colorB: string;
};

export const Blob = ({ colorA, colorB }: BlobProps) => {
  const gradientId = `${useId()}-blob-gradient`;

  return (
    <svg viewBox="0 0 1283 1306" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M466.346 299.976C775.983 219.838 1045.36 486.143 880.076 759.963C812.669 871.63 814.235 1123.57 687.961 1156.25C551.255 1191.63 533.392 1008.93 439.932 907.83C321.706 779.933 152.565 708.678 244.878 555.75C312.285 444.083 340.071 332.657 466.346 299.976Z"
        fill={`url(#${gradientId})`}
      />
      <path
        d="M491.752 293.511C791.06 180.762 1087.34 416.775 952.242 706.682C897.147 824.909 925.611 1075.24 803.549 1121.22C671.404 1171 634.132 991.255 530.408 900.71C399.2 786.171 223.416 733.387 298.868 571.476C353.963 453.248 369.691 339.492 491.752 293.511Z"
        stroke={colorB}
        strokeWidth="4"
        strokeLinecap="round"
        strokeDasharray="8 16"
      />
      <path
        d="M463.111 260.011C780.284 218.801 1014.64 516.392 816.746 767.655C736.04 870.124 706.421 1120.32 577.073 1137.12C437.04 1155.32 441.92 971.818 361.688 859.926C260.196 718.383 101.171 626.747 211.697 486.417C292.403 383.948 333.763 276.817 463.111 260.011Z"
        stroke={colorA}
        strokeWidth="4"
        strokeLinecap="round"
        strokeDasharray="8 16"
      />
      <defs>
        <linearGradient
          id={gradientId}
          x1="544.639"
          y1="372.03"
          x2="873.639"
          y2="1024.03"
          gradientUnits="userSpaceOnUse"
        >
          <stop stopColor={colorA} />
          <stop offset="1" stopColor={colorB} />
        </linearGradient>
      </defs>
    </svg>
  );
};

export default Blob;
