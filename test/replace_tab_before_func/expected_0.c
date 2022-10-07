unsigned int	get_str_size(char *array)
{
	int i;

	i = 0;
	while (array[i] != '\0')
		i++;
	return (i);
}

unsigned int	ft_strlcpy(char *dest, char *src, unsigned int size)
{
	unsigned int i;
	int j;
	char tmp;

	i = 0;
	while (i < size - 1)
	{
		dest[i] = src[i];
		i++;
	}
	dest[i] = '\0';
	return (get_str_size(src));
}
