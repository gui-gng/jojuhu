class ApiResponse<T> {
  final bool success;
  final T? data;
  final String? message;
  final String? error;
  final int? statusCode;

  ApiResponse({
    required this.success,
    this.data,
    this.message,
    this.error,
    this.statusCode,
  });

  factory ApiResponse.fromJson(
    Map<String, dynamic> json,
    T Function(dynamic)? parser,
  ) {
    return ApiResponse(
      success: json['success'] ?? false,
      data: parser != null && json['data'] != null ? parser(json['data']) : null,
      message: json['message'],
      error: json['error'],
    );
  }

  bool get hasError => !success || error != null;
}

class PaginatedResponse<T> {
  final List<T> items;
  final int page;
  final int perPage;
  final bool hasMore;

  PaginatedResponse({
    required this.items,
    required this.page,
    required this.perPage,
    required this.hasMore,
  });

  factory PaginatedResponse.fromList(
    List<dynamic> jsonList,
    T Function(dynamic) parser, {
    int page = 1,
    int perPage = 20,
  }) {
    final items = jsonList.map((item) => parser(item)).toList();
    return PaginatedResponse(
      items: items,
      page: page,
      perPage: perPage,
      hasMore: items.length >= perPage,
    );
  }
}
