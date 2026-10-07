| Trạng thái | Ký tự khuyên dùng | Ký tự thay thế | Mã Unicode | Màu ANSI đề xuất |
|-----------|-------------------|----------------|------------|-----------------|
| Success | ✔ | "✓, √" | U+2714 / U+2713 | \e[32m (Xanh lá) |
| Error / Fail | ✗ | "✖, ×" | U+2716 / U+2717 | \e[31m (Đỏ) |
| Warning | ⚠ | "!, ▲" | U+25B2 / U+0021 | \e[33m (Vàng) |
| Run / Exec | ▶ | "▸" | U+25B6 / U+279C | \e[36m (Cyan) hoặc \e[34m (Xanh dương) |
| Build / Work | ⚙ | "⛭, ◈, ◆" | U+2699 / U+25C6 | \e[35m (Tím magenta) |
| Info | ℹ | "i, ⓘ" | U+2139 / U+25CF | \e[34m (Xanh dương) |
| Pending / Wait | [·, •, ●, •] | "◌, ○" | U+25CC / U+25CB | \e[90m (Xám) |
| Pause | ⏸ | "⏸" | U+23F8 / U+2016 | \e[90m (Xám) |
| Stop | ■ | "◼" | U+25A0 | \e[37m (Trắng) |

| Ký tự | Tên Unicode | Kích thước tương đối |
| --- | --- | --- |
| · | Middle Dot (U+00B7) | Rất nhỏ (Tiny) |
| • | Bullet (U+2022) | Nhỏ (Small) |
| ● | Black Circle (U+25CF) | Vừa / Lớn | (Medium) |
| ⬤ | Black Large Circle (U+2B24) | Rất lớn  |(Large)

### Pendding
```PowerShell
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::CursorVisible = $false

$pulse = @("·", "•", "●", "•")

try {
    Write-Host "Đang xử lý dữ liệu..."
    while ($true) {
        foreach ($dot in $pulse) {
            Write-Host -NoNewline "`r$([char]27)[32m$dot$([char]27)[0m Đang tải dữ liệu, vui lòng đợi..."
            Start-Sleep -Milliseconds 150
        }
    }
} finally {
    [Console]::CursorVisible = $true
    Write-Host ""
}
```