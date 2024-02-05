export const COMMON_REGEX = {
  /**
   * Minimum eight characters, at least one letter, one number and one special character:
   */
  password: String.raw`^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$`,
};
