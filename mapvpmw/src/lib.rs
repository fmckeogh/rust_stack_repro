#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Zeros::*;
use fmod_int::*;
use fdiv_int::*;
use common::*;
pub fn mapvpmw<T: Tracer>(state: &mut State, tracer: &T, vpartid: i128) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        wd: i128,
        vpmw: u64,
        vpartid: i128,
    }
    let fn_state = FunctionState {
        vpartid,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_0_0: const #4s : i
        let s_0_0: i128 = 4;
        // D s_0_1: read-var vpartid:i
        let s_0_1: i128 = fn_state.vpartid;
        // D s_0_2: call fdiv_int(s_0_1, s_0_0)
        let s_0_2: i128 = fdiv_int(state, tracer, s_0_1, s_0_0);
        // D s_0_3: write-var wd <= s_0_2
        fn_state.wd = s_0_2;
        // D s_0_4: read-var wd:i
        let s_0_4: i128 = fn_state.wd;
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // D s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b3 b1
        if s_0_7 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_1_0: const #100224u : u32
        let s_1_0: u32 = 100224;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var vpmw <= s_1_1
        fn_state.vpmw = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_2_0: read-var vpmw:u64
        let s_2_0: u64 = fn_state.vpmw;
        // C s_2_1: const #4s : i
        let s_2_1: i128 = 4;
        // D s_2_2: read-var vpartid:i
        let s_2_2: i128 = fn_state.vpartid;
        // D s_2_3: call fmod_int(s_2_2, s_2_1)
        let s_2_3: i128 = fmod_int(state, tracer, s_2_2, s_2_1);
        // C s_2_4: const #16s : i
        let s_2_4: i128 = 16;
        // D s_2_5: mul s_2_3 s_2_4
        let s_2_5: i128 = ((s_2_3) * (s_2_4));
        // C s_2_6: const #16s : i
        let s_2_6: i128 = 16;
        // D s_2_7: cast zx s_2_0 -> bv
        let s_2_7: Bits = Bits::new(s_2_0 as u128, 64u16);
        // D s_2_8: bit-extract s_2_7 s_2_5 s_2_6
        let s_2_8: Bits = (Bits::new(
            ((s_2_7) >> (s_2_5)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_9: cast reint s_2_8 -> u16
        let s_2_9: u16 = (s_2_8.value() as u16);
        // N s_2_10: return s_2_9
        return s_2_9;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_3_0: read-var wd:i
        let s_3_0: i128 = fn_state.wd;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_4_0: const #10648u : u32
        let s_4_0: u32 = 10648;
        // D s_4_1: read-reg s_4_0:u64
        let s_4_1: u64 = {
            let value = state.read_register::<u64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: write-var vpmw <= s_4_1
        fn_state.vpmw = s_4_1;
        // N s_4_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_5_0: read-var wd:i
        let s_5_0: i128 = fn_state.wd;
        // C s_5_1: const #2s : i
        let s_5_1: i128 = 2;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_6_0: const #15464u : u32
        let s_6_0: u32 = 15464;
        // D s_6_1: read-reg s_6_0:u64
        let s_6_1: u64 = {
            let value = state.read_register::<u64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: write-var vpmw <= s_6_1
        fn_state.vpmw = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_7_0: read-var wd:i
        let s_7_0: i128 = fn_state.wd;
        // C s_7_1: const #3s : i
        let s_7_1: i128 = 3;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_8_0: const #90864u : u32
        let s_8_0: u32 = 90864;
        // D s_8_1: read-reg s_8_0:u64
        let s_8_1: u64 = {
            let value = state.read_register::<u64>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: write-var vpmw <= s_8_1
        fn_state.vpmw = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_9_0: read-var wd:i
        let s_9_0: i128 = fn_state.wd;
        // C s_9_1: const #4s : i
        let s_9_1: i128 = 4;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_10_0: const #22520u : u32
        let s_10_0: u32 = 22520;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: write-var vpmw <= s_10_1
        fn_state.vpmw = s_10_1;
        // N s_10_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_11_0: read-var wd:i
        let s_11_0: i128 = fn_state.wd;
        // C s_11_1: const #5s : i
        let s_11_1: i128 = 5;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_12_0: const #100984u : u32
        let s_12_0: u32 = 100984;
        // D s_12_1: read-reg s_12_0:u64
        let s_12_1: u64 = {
            let value = state.read_register::<u64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var vpmw <= s_12_1
        fn_state.vpmw = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_13_0: read-var wd:i
        let s_13_0: i128 = fn_state.wd;
        // C s_13_1: const #6s : i
        let s_13_1: i128 = 6;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_14_0: const #22544u : u32
        let s_14_0: u32 = 22544;
        // D s_14_1: read-reg s_14_0:u64
        let s_14_1: u64 = {
            let value = state.read_register::<u64>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: write-var vpmw <= s_14_1
        fn_state.vpmw = s_14_1;
        // N s_14_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_15_0: read-var wd:i
        let s_15_0: i128 = fn_state.wd;
        // C s_15_1: const #7s : i
        let s_15_1: i128 = 7;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_16_0: const #20984u : u32
        let s_16_0: u32 = 20984;
        // D s_16_1: read-reg s_16_0:u64
        let s_16_1: u64 = {
            let value = state.read_register::<u64>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: write-var vpmw <= s_16_1
        fn_state.vpmw = s_16_1;
        // N s_16_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_17_0: const #64s : i
        let s_17_0: i128 = 64;
        // S s_17_1: call Zeros(s_17_0)
        let s_17_1: Bits = Zeros(state, tracer, s_17_0);
        // S s_17_2: cast reint s_17_1 -> u64
        let s_17_2: u64 = (s_17_1.value() as u64);
        // D s_17_3: write-var vpmw <= s_17_2
        fn_state.vpmw = s_17_2;
        // N s_17_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
