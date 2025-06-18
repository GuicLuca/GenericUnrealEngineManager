// Helper function that returns a number of bytes in human-readable format
// Size: number,  amount of bytes.
export function formatSize(size: number): string {
    if (size < 1024) return `${size} B`
    if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`
    if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(2)} MB`
    return `${(size / (1024 * 1024 * 1024)).toFixed(2)} GB`
}

// Helper function that returns a number of seconds in human-readable format
// using the format: "HH:MM:SS"
export function formatDate(date: number): string {
    const hours = Math.floor(date / 3600);
    const minutes = Math.floor((date % 3600) / 60);
    const seconds = date % 60;

    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
}

// Helper function that returns the time since a given date in human-readable format
// date: number, the number of seconds since the UNIX_epoch
export function timeSince(date: number): string {
    const seconds = Math.floor((Date.now() - date * 1000) / 1000);
    const intervals = [
        { label: 'year', seconds: 31536000 },
        { label: 'month', seconds: 2592000 },
        { label: 'day', seconds: 86400 },
        { label: 'hour', seconds: 3600 },
        { label: 'minute', seconds: 60 },
        { label: 'second', seconds: 1 }
    ];

    for (const interval of intervals) {
        const count = Math.floor(seconds / interval.seconds);
        if (count > 0) {
            return `${count} ${interval.label}${count > 1 ? 's' : ''} ago`;
        }
    }
    return 'just now';
}