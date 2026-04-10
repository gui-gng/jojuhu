import 'package:flutter/material.dart';
import 'package:jojuhu/theme/jojuhu_theme.dart';
import 'package:jojuhu/theme/tokens.dart';
import 'package:jojuhu/widgets/jojuhu_text_field.dart';

class SearchScreen extends StatefulWidget {
  const SearchScreen({super.key});

  @override
  State<SearchScreen> createState() => _SearchScreenState();
}

class _SearchScreenState extends State<SearchScreen> with SingleTickerProviderStateMixin {
  final TextEditingController _searchController = TextEditingController();
  late TabController _tabController;
  String _searchType = 'users'; // 'users', 'posts', 'hashtags'
  bool _hasSearched = false;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
    _tabController.addListener(() {
      if (_tabController.indexIsChanging) return;
      setState(() {
        _searchType = ['users', 'posts', 'hashtags'][_tabController.index];
      });
    });
  }

  @override
  void dispose() {
    _searchController.dispose();
    _tabController.dispose();
    super.dispose();
  }

  void _handleSearch(String query) {
    if (query.trim().isEmpty) {
      setState(() {
        _hasSearched = false;
      });
      return;
    }
    
    setState(() {
      _hasSearched = true;
    });
    
    // TODO: Implement actual search when API endpoints are available
    // For now, just show placeholder results
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Search'),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'Users'),
            Tab(text: 'Posts'),
            Tab(text: 'Hashtags'),
          ],
          labelColor: JojuhuColors.sol,
          unselectedLabelColor: JojuhuColors.textoCinza,
          indicatorColor: JojuhuColors.sol,
        ),
      ),
      body: Column(
        children: [
          // Search bar
          Padding(
            padding: JojuhuSpacing.paddingScreen,
            child: JojuhuTextField(
              hint: 'Search $_searchType...',
              prefixIcon: Icons.search,
              suffixIcon: _searchController.text.isNotEmpty ? Icons.clear : null,
              onSuffixTap: () {
                _searchController.clear();
                setState(() {
                  _hasSearched = false;
                });
              },
              controller: _searchController,
              onChanged: _handleSearch,
              textInputAction: TextInputAction.search,
            ),
          ),

          // Results
          Expanded(
            child: _hasSearched
                ? _buildResultsPlaceholder()
                : _buildEmptyState(),
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyState() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.search,
            size: 64,
            color: JojuhuColors.textoCinza,
          ),
          const SizedBox(height: JojuhuSpacing.lg),
          Text(
            'Search for $_searchType',
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                  color: JojuhuColors.textoCinza,
                ),
          ),
        ],
      ),
    );
  }

  Widget _buildResultsPlaceholder() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.search_off,
            size: 48,
            color: JojuhuColors.textoCinza,
          ),
          const SizedBox(height: JojuhuSpacing.md),
          Text(
            'No results found',
            style: Theme.of(context).textTheme.titleMedium,
          ),
          const SizedBox(height: JojuhuSpacing.sm),
          Text(
            'Try a different search term',
            style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: JojuhuColors.textoCinza,
                ),
          ),
        ],
      ),
    );
  }
}