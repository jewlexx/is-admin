namespace network
{
    INTERNET_STATUS IsConnectedToNetwork();

    enum class INTERNET_STATUS
    {
        CONNECTED,
        DISCONNECTED,
        CONNECTED_TO_LOCAL,
        CONNECTION_ERROR
    };
}