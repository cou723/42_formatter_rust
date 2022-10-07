#ifndef LIBFT_H
# define LIBFT_H

# include <errno.h>
# include <limits.h>
# include <stddef.h>
# include <stdlib.h>
# include <unistd.h>

int				ft_toupper(int c);
int				ft_tolower(int c);
char			*ft_substr(char const *str, unsigned int start, size_t len);
char			*ft_strtrim(char const *s1, char const *set);
char			*ft_strrchr(const char *s, int c);
int				ft_strncmp(const char *s1, const char *s2, size_t len);
char			*ft_strmapi(char const *s, char (*f)(unsigned int, char));
size_t			ft_strlen(char *str);
unsigned int	ft_strlcpy(char *dest, char *src, unsigned int size);
size_t			ft_strlcat(char *dest, char *src, size_t size);
char			*ft_strjoin(char const *s1, char const *s2);
void			ft_striteri(char *s, void (*f)(unsigned int, char *));
char			*ft_strdup(const char *str);
char			*ft_strchr(const char *s, int c);
char			**ft_split(char const *s, char c);
void			ft_putstr_fd(char *s, int fd);
void			ft_putnbr_fd(int n, int fd);
void			ft_putendl_fd(char *s, int fd);
void			ft_putchar_fd(char c, int fd);
void			*ft_memset(void *s, int c, size_t n);
void			*ft_memmove(void *dest, const void *src, size_t n);
void			*ft_memcpy(void *dest, const void *src, size_t n);
int				ft_memcmp(const void *s1, const void *s2, size_t n);
void			*ft_memchr(const void *point, int c, size_t n);
char			*ft_itoa(int n);
int				ft_isprint(int c);
int				ft_isdigit(int c);
int				ft_isascii(int c);
int				ft_isalpha(int c);
int				ft_isalnum(int c);

void			ft_bzero(void *s, size_t n);
int				ft_atoi(char *str);
char			*ft_strnstr(const char *big, const char *little, size_t len);
char			**ft_split(char const *str, char c);
void			*ft_calloc(size_t count, size_t size);

#endif