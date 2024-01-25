const colors = require("tailwindcss/colors");

/** @type {import('tailwindcss').Config} */
export default {
  content: ["*.html", "*.css"],
  theme: {
    colors: {
      navigationBg: "var(--navigationBg)",
      navigationText: "var(--navigationText)",
      onNeutralBg: "var(--onNeutralBg)",
      neutralBg: "var(--neutralBg)",
      onPrimaryBg: "var(--onPrimaryBg)",
      primaryBg: "var(--primaryBg)",
      textColor: "var(--textColor)",
    },
    extend: {
      typography: ({ theme }) => ({
        default: {
          css: {
            "--tw-prose-body": "var(--prose-body)",
            "--tw-prose-headings": "var(--prose-headings)",
            "--tw-prose-lead": "var(--prose-lead)",
            "--tw-prose-links": "var(--prose-links)",
            "--tw-prose-bold": "var(--prose-bold)",
            "--tw-prose-counters": "var(--prose-counters)",
            "--tw-prose-bullets": "var(--prose-bullets)",
            "--tw-prose-hr": "var(--prose-hr)",
            "--tw-prose-quotes": "var(--prose-quotes)",
            "--tw-prose-quote-borders": "var(--prose-quote-borders)",
            "--tw-prose-captions": "var(--prose-captions)",
            "--tw-prose-code": "var(--prose-code)",
            "--tw-prose-pre-code": "var(--prose-pre-code)",
            "--tw-prose-pre-bg": "var(--prose-pre-bg)",
            "--tw-prose-th-borders": "var(--prose-th-borders)",
            "--tw-prose-td-borders": "var(--prose-td-borders)",
          },
        },
      }),
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
