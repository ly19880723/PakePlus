// 图标处理工具 - 生成各平台所需图标
// 利用 Canvas 在前端生成不同尺寸的图标

// 将图片文件转为 base64
export function fileToBase64(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = () => resolve(reader.result as string)
        reader.onerror = reject
        reader.readAsDataURL(file)
    })
}

// 从 base64 创建 Image 对象
export function loadImage(src: string): Promise<HTMLImageElement> {
    return new Promise((resolve, reject) => {
        const img = new Image()
        img.onload = () => resolve(img)
        img.onerror = reject
        img.src = src
    })
}

// 将 Image 绘制到指定尺寸的 Canvas 并返回 base64
export function resizeImage(img: HTMLImageElement, size: number, rounded: boolean = false): string {
    const canvas = document.createElement('canvas')
    canvas.width = size
    canvas.height = size
    const ctx = canvas.getContext('2d')!
    ctx.clearRect(0, 0, size, size)

    if (rounded) {
        // 圆角裁剪
        const radius = size / 2
        ctx.beginPath()
        ctx.arc(size / 2, size / 2, radius, 0, Math.PI * 2)
        ctx.closePath()
        ctx.clip()
    }

    // 居中绘制
    ctx.drawImage(img, 0, 0, size, size)
    return canvas.toDataURL('image/png')
}

// 生成各平台图标尺寸
export interface PlatformIcons {
    // Android
    android: { size: number; name: string }[]
    // iOS
    ios: { size: number; name: string }[]
    // 桌面端
    desktop: { size: number; name: string }[]
}

// 平台图标尺寸定义
export const ICON_SIZES: PlatformIcons = {
    android: [
        { size: 48, name: 'mipmap-mdpi' },
        { size: 72, name: 'mipmap-hdpi' },
        { size: 96, name: 'mipmap-xhdpi' },
        { size: 144, name: 'mipmap-xxhdpi' },
        { size: 192, name: 'mipmap-xxxhdpi' },
    ],
    ios: [
        { size: 20, name: '20x20@1x' },
        { size: 40, name: '20x20@2x' },
        { size: 60, name: '20x20@3x' },
        { size: 29, name: '29x29@1x' },
        { size: 58, name: '29x29@2x' },
        { size: 87, name: '29x29@3x' },
        { size: 40, name: '40x40@1x' },
        { size: 80, name: '40x40@2x' },
        { size: 120, name: '40x40@3x' },
        { size: 76, name: '76x76@1x' },
        { size: 152, name: '76x76@2x' },
        { size: 167, name: '83.5x83.5@2x' },
        { size: 120, name: '60x60@2x' },
        { size: 180, name: '60x60@3x' },
        { size: 1024, name: '512@2x' },
    ],
    desktop: [
        { size: 16, name: '16x16' },
        { size: 32, name: '32x32' },
        { size: 64, name: '64x64' },
        { size: 128, name: '128x128' },
        { size: 256, name: '256x256' },
        { size: 512, name: '512x512' },
    ],
}

// 生成所有平台图标
export async function generateAllIcons(
    base64Png: string,
    rounded: boolean = false
): Promise<Record<string, string>> {
    const img = await loadImage(base64Png)
    const result: Record<string, string> = {}

    for (const platform of Object.keys(ICON_SIZES)) {
        for (const { size, name } of ICON_SIZES[platform as keyof PlatformIcons]) {
            result[`${platform}/${name}`] = resizeImage(img, size, rounded)
        }
    }
    return result
}

// 生成单个尺寸图标
export async function generateIcon(
    base64Png: string,
    size: number,
    rounded: boolean = false
): Promise<string> {
    const img = await loadImage(base64Png)
    return resizeImage(img, size, rounded)
}
