-- Note: To make it work use `set ex` in the nvim config
-- Note: add this function to ~/.config/nvim/config.lua:
--
-- ```lua
-- vim.g.load_rust_lsp = function(settings)
--    require 'lspconfig'.rust_analyzer.setup {
--       on_attach = CUSTOM_ATTACH,
--       settings = {
--          ["rust-analyzer"] = settings,
--       },
--       tools = {
--          inlay_hints = {
--             auto = true
--          }
--       },
--    }
-- end
-- ```

if vim.g.load_rust_lsp ~= nil then
   vim.g.load_rust_lsp({
      cargo = {
         features = { "serde", "sys-info" },
         buildScripts = {
            enable = true,
         },
      },
      trace = {
         server = "verbose",
      },
      diagnostics = {
         enable = true,
         disabled = { "proc-macro-disabled" },
         enableExperimental = true,
      },
      procMacro = {
         enable = true,
         ignored = {
            -- This is required, because of:
            -- https://github.com/rust-lang/rust-analyzer/issues/15800
            cfg_eval = { "cfg_eval" },
         }
      },
   })
end
