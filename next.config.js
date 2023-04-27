/** @type {import('next').NextConfig} */
/* eslint-env node */
module.exports = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    unoptimized: true,
  },
  output: "export",
  distDir: "dist",
  experimental: {
    appDir: true,
  },
};
