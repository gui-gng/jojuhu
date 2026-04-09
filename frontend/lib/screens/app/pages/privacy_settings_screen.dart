import 'package:flutter/material.dart';
import 'package:jojuhu_frontend/models/api_response.dart';
import 'package:jojuhu_frontend/services/api_service.dart';

class PrivacySettingsScreen extends StatefulWidget {
  const PrivacySettingsScreen({super.key});

  @override
  State<PrivacySettingsScreen> createState() => _PrivacySettingsScreenState();
}

class _PrivacySettingsScreenState extends State<PrivacySettingsScreen> {
  bool _isLoading = true;
  bool _isSaving = false;
  String? _error;

  // Privacy settings
  String _profileVisibility = 'public';
  bool _showEmail = false;
  bool _showPhone = false;
  bool _allowMentions = true;
  bool _allowTags = true;
  bool _showOnlineStatus = true;
  bool _showActivity = true;
  bool _allowSearchEngines = false;
  bool _dataProcessingConsent = false;
  bool _marketingEmailsConsent = false;

  // Data export
  Map<String, dynamic>? _exportRequest;
  bool _isExporting = false;

  // Account deletion
  Map<String, dynamic>? _deletionRequest;
  bool _isDeleting = false;
  final TextEditingController _deletionReasonController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadSettings();
  }

  Future<void> _loadSettings() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final response = await ApiService.getPrivacySettings();
      if (response.success && response.data != null) {
        final data = response.data!;
        setState(() {
          _profileVisibility = data['profile_visibility'] ?? 'public';
          _showEmail = data['show_email'] ?? false;
          _showPhone = data['show_phone'] ?? false;
          _allowMentions = data['allow_mentions'] ?? true;
          _allowTags = data['allow_tags'] ?? true;
          _showOnlineStatus = data['show_online_status'] ?? true;
          _showActivity = data['show_activity'] ?? true;
          _allowSearchEngines = data['allow_search_engines'] ?? false;
          _dataProcessingConsent = data['data_processing_consent'] ?? false;
          _marketingEmailsConsent = data['marketing_emails_consent'] ?? false;
        });
      } else {
        setState(() {
          _error = response.error ?? 'Failed to load settings';
        });
      }
    } catch (e) {
      setState(() {
        _error = 'Error: $e';
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _updateSetting(String key, dynamic value) async {
    if (_isSaving) return;

    setState(() {
      _isSaving = true;
    });

    try {
      final updates = <String, dynamic>{key: value};
      final response = await ApiService.updatePrivacySettings(updates);
      
      if (response.success) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Settings updated')),
        );
      } else {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(response.error ?? 'Failed to update')),
        );
        // Revert the change
        await _loadSettings();
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    } finally {
      setState(() {
        _isSaving = false;
      });
    }
  }

  Future<void> _requestDataExport() async {
    if (_isExporting) return;

    setState(() {
      _isExporting = true;
    });

    try {
      final response = await ApiService.requestDataExport();
      if (response.success) {
        setState(() {
          _exportRequest = response.data;
        });
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Data export requested. You will be notified when ready.')),
        );
      } else {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(response.error ?? 'Failed to request export')),
        );
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    } finally {
      setState(() {
        _isExporting = false;
      });
    }
  }

  Future<void> _requestAccountDeletion() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Account'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'This action cannot be undone. Your account will be scheduled for deletion in 30 days.',
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _deletionReasonController,
              decoration: const InputDecoration(
                labelText: 'Reason (optional)',
                border: OutlineInputBorder(),
              ),
              maxLines: 2,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.pop(context, true),
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.red,
            ),
            child: const Text('Delete'),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    setState(() {
      _isDeleting = true;
    });

    try {
      final response = await ApiService.requestAccountDeletion(
        reason: _deletionReasonController.text.isNotEmpty
            ? _deletionReasonController.text
            : null,
      );

      if (response.success) {
        setState(() {
          _deletionRequest = response.data;
        });
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Account deletion requested')),
        );
      } else {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(response.error ?? 'Failed to request deletion')),
        );
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    } finally {
      setState(() {
        _isDeleting = false;
      });
    }
  }

  Future<void> _cancelDeletion() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Cancel Deletion'),
        content: const Text('Are you sure you want to cancel the account deletion request?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('No'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text('Yes'),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    try {
      final response = await ApiService.cancelAccountDeletion();
      if (response.success) {
        setState(() {
          _deletionRequest = null;
        });
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Account deletion cancelled')),
        );
      } else {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(response.error ?? 'Failed to cancel')),
        );
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Privacy Settings'),
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _error != null
              ? Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(_error!),
                      ElevatedButton(
                        onPressed: _loadSettings,
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : ListView(
                  padding: const EdgeInsets.all(16),
                  children: [
                    // Profile Visibility
                    _buildSection(
                      'Profile Visibility',
                      [
                        ListTile(
                          title: const Text('Profile Visibility'),
                          subtitle: Text(_profileVisibility == 'public'
                              ? 'Public - Anyone can see your profile'
                              : _profileVisibility == 'followers_only'
                                  ? 'Followers Only - Only followers can see your profile'
                                  : 'Private - Only you can see your profile'),
                          trailing: DropdownButton<String>(
                            value: _profileVisibility,
                            onChanged: (value) {
                              if (value != null) {
                                setState(() => _profileVisibility = value);
                                _updateSetting('profile_visibility', value);
                              }
                            },
                            items: const [
                              DropdownMenuItem(value: 'public', child: Text('Public')),
                              DropdownMenuItem(value: 'followers_only', child: Text('Followers Only')),
                              DropdownMenuItem(value: 'private', child: Text('Private')),
                            ],
                          ),
                        ),
                      ],
                    ),

                    const Divider(),

                    // Contact Information
                    _buildSection(
                      'Contact Information',
                      [
                        SwitchListTile(
                          title: const Text('Show Email'),
                          subtitle: const Text('Allow others to see your email address'),
                          value: _showEmail,
                          onChanged: (value) {
                            setState(() => _showEmail = value);
                            _updateSetting('show_email', value);
                          },
                        ),
                        SwitchListTile(
                          title: const Text('Show Phone'),
                          subtitle: const Text('Allow others to see your phone number'),
                          value: _showPhone,
                          onChanged: (value) {
                            setState(() => _showPhone = value);
                            _updateSetting('show_phone', value);
                          },
                        ),
                      ],
                    ),

                    const Divider(),

                    // Interactions
                    _buildSection(
                      'Interactions',
                      [
                        SwitchListTile(
                          title: const Text('Allow Mentions'),
                          subtitle: const Text('Allow others to mention you with @'),
                          value: _allowMentions,
                          onChanged: (value) {
                            setState(() => _allowMentions = value);
                            _updateSetting('allow_mentions', value);
                          },
                        ),
                        SwitchListTile(
                          title: const Text('Allow Tags'),
                          subtitle: const Text('Allow others to tag you in posts'),
                          value: _allowTags,
                          onChanged: (value) {
                            setState(() => _allowTags = value);
                            _updateSetting('allow_tags', value);
                          },
                        ),
                      ],
                    ),

                    const Divider(),

                    // Activity Status
                    _buildSection(
                      'Activity Status',
                      [
                        SwitchListTile(
                          title: const Text('Show Online Status'),
                          subtitle: const Text('Let others see when you are online'),
                          value: _showOnlineStatus,
                          onChanged: (value) {
                            setState(() => _showOnlineStatus = value);
                            _updateSetting('show_online_status', value);
                          },
                        ),
                        SwitchListTile(
                          title: const Text('Show Activity'),
                          subtitle: const Text('Show your activity status to others'),
                          value: _showActivity,
                          onChanged: (value) {
                            setState(() => _showActivity = value);
                            _updateSetting('show_activity', value);
                          },
                        ),
                      ],
                    ),

                    const Divider(),

                    // Search & Discovery
                    _buildSection(
                      'Search & Discovery',
                      [
                        SwitchListTile(
                          title: const Text('Allow Search Engines'),
                          subtitle: const Text('Allow your profile to appear in search engines'),
                          value: _allowSearchEngines,
                          onChanged: (value) {
                            setState(() => _allowSearchEngines = value);
                            _updateSetting('allow_search_engines', value);
                          },
                        ),
                      ],
                    ),

                    const Divider(),

                    // Consent
                    _buildSection(
                      'Consent',
                      [
                        SwitchListTile(
                          title: const Text('Data Processing Consent'),
                          subtitle: const Text('I consent to data processing'),
                          value: _dataProcessingConsent,
                          onChanged: (value) {
                            setState(() => _dataProcessingConsent = value);
                            _updateSetting('data_processing_consent', value);
                          },
                        ),
                        SwitchListTile(
                          title: const Text('Marketing Emails'),
                          subtitle: const Text('I consent to receive marketing emails'),
                          value: _marketingEmailsConsent,
                          onChanged: (value) {
                            setState(() => _marketingEmailsConsent = value);
                            _updateSetting('marketing_emails_consent', value);
                          },
                        ),
                      ],
                    ),

                    const SizedBox(height: 24),

                    // Data Export Section
                    _buildSection(
                      'GDPR Data Rights',
                      [
                        Card(
                          child: Padding(
                            padding: const EdgeInsets.all(16),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                const Text(
                                  'Export Your Data',
                                  style: TextStyle(
                                    fontSize: 16,
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                                const SizedBox(height: 8),
                                const Text(
                                  'Request a copy of all your data (posts, messages, etc.)',
                                ),
                                const SizedBox(height: 16),
                                if (_exportRequest != null) ...[
                                  Text(
                                    'Status: ${_exportRequest!['status']}',
                                    style: const TextStyle(fontWeight: FontWeight.bold),
                                  ),
                                  const SizedBox(height: 8),
                                ],
                                ElevatedButton.icon(
                                  onPressed: _isExporting ? null : _requestDataExport,
                                  icon: _isExporting
                                      ? const SizedBox(
                                          width: 16,
                                          height: 16,
                                          child: CircularProgressIndicator(strokeWidth: 2),
                                        )
                                      : const Icon(Icons.download),
                                  label: Text(_isExporting ? 'Exporting...' : 'Export Data'),
                                ),
                              ],
                            ),
                          ),
                        ),
                      ],
                    ),

                    const SizedBox(height: 16),

                    // Account Deletion Section
                    _buildSection(
                      'Danger Zone',
                      [
                        Card(
                          color: Colors.red.shade50,
                          child: Padding(
                            padding: const EdgeInsets.all(16),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                const Text(
                                  'Delete Account',
                                  style: TextStyle(
                                    fontSize: 16,
                                    fontWeight: FontWeight.bold,
                                    color: Colors.red,
                                  ),
                                ),
                                const SizedBox(height: 8),
                                const Text(
                                  'Permanently delete your account and all data. This action cannot be undone.',
                                  style: TextStyle(color: Colors.red),
                                ),
                                const SizedBox(height: 16),
                                if (_deletionRequest != null) ...[
                                  Text(
                                    'Deletion scheduled for: ${_deletionRequest!['scheduled_deletion_at']}',
                                    style: const TextStyle(fontWeight: FontWeight.bold),
                                  ),
                                  const SizedBox(height: 8),
                                ],
                                if (_deletionRequest != null && _deletionRequest!['status'] == 'pending')
                                  ElevatedButton.icon(
                                    onPressed: _cancelDeletion,
                                    icon: const Icon(Icons.cancel),
                                    label: const Text('Cancel Deletion'),
                                    style: ElevatedButton.styleFrom(
                                      backgroundColor: Colors.orange,
                                    ),
                                  )
                                else
                                  ElevatedButton.icon(
                                    onPressed: _isDeleting ? null : _requestAccountDeletion,
                                    icon: const Icon(Icons.delete_forever),
                                    label: Text(_isDeleting ? 'Processing...' : 'Delete Account'),
                                    style: ElevatedButton.styleFrom(
                                      backgroundColor: Colors.red,
                                    ),
                                  ),
                              ],
                            ),
                          ),
                        ),
                      ],
                    ),

                    const SizedBox(height: 32),
                  ],
                ),
    );
  }

  Widget _buildSection(String title, List<Widget> children) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          child: Text(
            title,
            style: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        ...children,
      ],
    );
  }

  @override
  void dispose() {
    _deletionReasonController.dispose();
    super.dispose();
  }
}