initSidebarItems({"constant":[["IN_ACCESS","Event: File was accessed.When monitoring a directory, the event may occur both for the directory itself and the files within."],["IN_ALL_EVENTS","Event: Any event occured."],["IN_ATTRIB","Event: Metadata has changed.This can include e.g. - permissions, see chmod(2); - timestamps, see utimensat(2); - extended attributes, see [setxattr(s)]; - link count, see link(2) and unlink(2); - user/group, see chown(2).When monitoring a directory, the event may occur both for the directory itself and the files within."],["IN_CLOEXEC","Flag: Set the FD_CLOEXEC flagThe FD_CLOEXEC flag, or \"close-on-exec\", changes the behavior of file descriptor when execve(2)'d:If the FD_CLOEXEC bit is 0, the file descriptor will remain open across an execve(2), otherwise it will be closed.See open(2) and fcntl(2) for details."],["IN_CLOSE","Event: File opened was closed.When monitoring a directory, the event may occur both for the directory itself and the files within."],["IN_CLOSE_NOWRITE","Event: File not opened for writing was closed.When monitoring a directory, the event may occur both for the directory itself and the files within."],["IN_CLOSE_WRITE","Event: File opened for writing was closed.When monitoring a directory, the event may occur both for the directory itself and the files within."],["IN_CREATE","Event: File or directory was created.This may also include hard links, symlinks, and UNIX sockets.When monitoring a directory, the event may occur *only* for the files within, not the directory itself."],["IN_DELETE","Event: File or directory was deleted.This may also include hard links, symlinks, and UNIX sockets.When monitoring a directory, the event may occur *only* for the files within, not the directory itself."],["IN_DELETE_SELF","Event: Watched file or directory was deleted.This may also occur if the object is moved to another filesystem, since mv(1) in effect copies the file to the other filesystem and then deletes it from the original.An IN_IGNORED event will subsequently be generated."],["IN_DONT_FOLLOW","Option: Don't dereference (if self is a symlink)."],["IN_EXCL_UNLINK","Option: Don't watch unlinked children.By default, when watching events on the children of a directory, events are generated for children even after they have been unlinked from the directory.  This can result in large numbers of uninteresting events for some applications (e.g., if watching /tmp, in which many applications create temporary files whose names are immediately unlinked).IN_EXCL_UNLINK changes this behavior, so that events are not generated for children after they have been unlinked from the watched directory."],["IN_IGNORED","Info: Watch was removed.This can occur either as a result of `inotify_rm_watch()`, or because self was deleted or the containing filesystem was unmounted, or after an IN_ONESHOT watch is complete.See the BUGS section of inotify(7) for more details."],["IN_ISDIR","Info: Subject of this event is a directory."],["IN_MASK_ADD","Option: Add events to an existing watch instead of replacing it.If a watch instance already exists for the filesystem object corresponding to self, add (|) the events to the watch mask instead of replacing it."],["IN_MODIFY",""],["IN_MOVE","Event: File or directory was moved away or in.When monitoring a directory, the event may occur *only* for the files within, not the directory itself."],["IN_MOVED_FROM","Event: File or directory was moved away.When monitoring a directory, the event may occur *only* for the files within, not the directory itself."],["IN_MOVED_TO","Event: File or directory was moved in.When monitoring a directory, the event may occur *only* for the files within, not the directory itself."],["IN_MOVE_SELF","Event: Watched file or directory was moved."],["IN_NONBLOCK","Flag: Set the O_NONBLOCK file status flagThe O_NONBLOCK flag changes the behavior of system calls when accessing files with mandatory locks:By default, both traditional (process-associated) and open file description record locks are advisory.  Advisory locks are not enforced and are useful only between cooperating processes.Both lock types can also be mandatory. Mandatory locks are enforced for all processes. If a process tries to perform an incompatible access (e.g., read(2) or write(2)) on a file region that has an incompatible mandatory lock, then the result depends upon whether the O_NONBLOCK flag is enabled for its open file description. If the O_NONBLOCK flag is not enabled, then the system call is blocked until the lock is removed or converted to a mode that is compatible with the access. If the O_NONBLOCK flag is enabled, then the system call fails with the error EAGAIN.See fcntl(2) for more details."],["IN_ONESHOT","Option: Listen for one event, then remove the watch."],["IN_ONLYDIR","Option: Don't watch children (if self is a directory)."],["IN_OPEN","Event: File was opened.When monitoring a directory, the event may occur both for the directory itself and the files within."],["IN_Q_OVERFLOW","Info: Event queue overflowed."],["IN_UNMOUNT","Info: Filesystem containing self was unmounted.An IN_IGNORED event will subsequently be generated."]],"fn":[["close",""],["inotify_add_watch","Manipulates the \"watch list\" associated with an inotify instance.Each item (\"watch\") in the watch list specifies the pathname of a file or directory, along with some set of events that the kernel should monitor for the file referred to by that pathname.This function either creates a new watch, or modifies an existing one."],["inotify_init","Creates an inotify instance.Returns a file descriptor referring to the inotify instance."],["inotify_init1","Creates an inotify instance.Also takes a bit mask of flags that provide access to extra functionality. Returns a file descriptor."],["inotify_rm_watch","Removes an item from an inotify watch list."],["read",""]],"struct":[["inotify_event","Describes an event.To determine what events have occurred, an application read(2)s from the inotify file descriptor.  If no events have so far occurred, then, assuming a blocking file descriptor, read(2) will block until at least one event occurs (unless interrupted by a signal, in which case the call fails with the error EINTR; see signal(7)).Each successful read(2) returns a buffer containing one or more of this structure."]]});