// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Outcome of a coroutine's start.
pub enum StartOutcome<Yields: Sized, Complete: Sized>
{
	/// Coroutine has returned an intermediate result and would to resume.
	WouldLikeToResume(Yields),

	/// Coroutine has completed.
	Complete(Complete),
}
