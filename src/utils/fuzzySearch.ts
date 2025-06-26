/**
 * Fuzzy search utilities for matching text with typo tolerance
 */

/**
 * Calculate Levenshtein distance between two strings
 */
function levenshteinDistance(str1: string, str2: string): number {
  const matrix = Array(str1.length + 1).fill(null).map(() => Array(str2.length + 1).fill(null))
  
  for (let i = 0; i <= str1.length; i++) {
    matrix[i][0] = i
  }
  
  for (let j = 0; j <= str2.length; j++) {
    matrix[0][j] = j
  }
  
  for (let i = 1; i <= str1.length; i++) {
    for (let j = 1; j <= str2.length; j++) {
      const cost = str1[i - 1] === str2[j - 1] ? 0 : 1
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1,      // deletion
        matrix[i][j - 1] + 1,      // insertion
        matrix[i - 1][j - 1] + cost // substitution
      )
    }
  }
  
  return matrix[str1.length][str2.length]
}

/**
 * Calculate similarity score between search term and target string
 * Returns a score between 0 and 1, where 1 is a perfect match
 */
export function calculateSimilarity(searchTerm: string, target: string): number {
  if (!searchTerm) return 1
  
  const search = searchTerm.toLowerCase().trim()
  const text = target.toLowerCase().trim()
  
  // Exact match gets highest score
  if (text === search) return 1
  
  // Substring match gets high score
  if (text.includes(search)) {
    // Score based on how much of the target string the search covers
    return 0.8 + (search.length / text.length) * 0.2
  }
  
  // Check if search starts with target or vice versa
  if (text.startsWith(search) || search.startsWith(text)) {
    return 0.7
  }
  
  // Calculate Levenshtein distance for fuzzy matching
  const distance = levenshteinDistance(search, text)
  const maxLength = Math.max(search.length, text.length)
  
  // Convert distance to similarity score (0-1)
  const similarity = 1 - (distance / maxLength)
  
  // Apply threshold - only consider matches with reasonable similarity
  return similarity > 0.5 ? similarity : 0
}

/**
 * Fuzzy search function that returns matching items with scores
 */
export function fuzzySearch<T>(
  items: T[],
  searchTerm: string,
  getSearchableText: (item: T) => string,
  threshold: number = 0.3
): Array<{ item: T; score: number }> {
  if (!searchTerm.trim()) {
    return items.map(item => ({ item, score: 1 }))
  }
  
  return items
    .map(item => ({
      item,
      score: calculateSimilarity(searchTerm, getSearchableText(item))
    }))
    .filter(result => result.score >= threshold)
    .sort((a, b) => b.score - a.score)
}

/**
 * Simple fuzzy match function for basic use cases
 */
export function fuzzyMatch(searchTerm: string, target: string, threshold: number = 0.3): boolean {
  return calculateSimilarity(searchTerm, target) >= threshold
}