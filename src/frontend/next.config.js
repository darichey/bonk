/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  experimental: {
    // FIXME: DashboardPage uses useSearchParams, and next cries because it de-opts the page, but it is already "use client". I'm probably doing something wrong but *shrug*
    missingSuspenseWithCSRBailout: false,
  },
};

module.exports = nextConfig;
