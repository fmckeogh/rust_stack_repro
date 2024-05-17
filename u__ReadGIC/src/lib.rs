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
use gic_read_ram::*;
use u__GIC_AcknowledgeInterrupt::*;
use Zeros::*;
use common::*;
pub fn u__ReadGIC<T: Tracer>(state: &mut State, tracer: &T, offset: u16) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_331318: u32,
        offset: u16,
    }
    let fn_state = FunctionState {
        offset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var offset:u16
        let s_0_0: u16 = fn_state.offset;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 16u16);
        // C s_0_2: const #4100u : u16
        let s_0_2: u16 = 4100;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 16u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b3 b1
        if s_0_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #1368u : u32
        let s_1_0: u32 = 1368;
        // D s_1_1: read-reg s_1_0:u32
        let s_1_1: u32 = {
            let value = state.read_register::<u32>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var gs#331318 <= s_1_1
        fn_state.gs_331318 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#331318:u32
        let s_2_0: u32 = fn_state.gs_331318;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var offset:u16
        let s_3_0: u16 = fn_state.offset;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 16u16);
        // C s_3_2: const #6144u : u16
        let s_3_2: u16 = 6144;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 16u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #4294967295u : u32
        let s_4_0: u32 = 4294967295;
        // D s_4_1: write-var gs#331318 <= s_4_0
        fn_state.gs_331318 = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var offset:u16
        let s_5_0: u16 = fn_state.offset;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 16u16);
        // C s_5_2: const #7172u : u16
        let s_5_2: u16 = 7172;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 16u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #7172u : u16
        let s_6_0: u16 = 7172;
        // S s_6_1: call gic_read_ram(s_6_0)
        let s_6_1: u32 = gic_read_ram(state, tracer, s_6_0);
        // C s_6_2: const #7172u : u16
        let s_6_2: u16 = 7172;
        // S s_6_3: call gic_read_ram(s_6_2)
        let s_6_3: u32 = gic_read_ram(state, tracer, s_6_2);
        // D s_6_4: write-var gs#331318 <= s_6_3
        fn_state.gs_331318 = s_6_3;
        // N s_6_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_7_0: read-var offset:u16
        let s_7_0: u16 = fn_state.offset;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 16u16);
        // C s_7_2: const #8192u : u16
        let s_7_2: u16 = 8192;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 16u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #15648u : u32
        let s_8_0: u32 = 15648;
        // D s_8_1: read-reg s_8_0:u32
        let s_8_1: u32 = {
            let value = state.read_register::<u32>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: write-var gs#331318 <= s_8_1
        fn_state.gs_331318 = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_9_0: read-var offset:u16
        let s_9_0: u16 = fn_state.offset;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 16u16);
        // C s_9_2: const #8204u : u16
        let s_9_2: u16 = 8204;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 16u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call __GIC_AcknowledgeInterrupt(s_10_0)
        let s_10_1: u32 = u__GIC_AcknowledgeInterrupt(state, tracer, s_10_0);
        // D s_10_2: write-var gs#331318 <= s_10_1
        fn_state.gs_331318 = s_10_1;
        // N s_10_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_11_0: read-var offset:u16
        let s_11_0: u16 = fn_state.offset;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 16u16);
        // C s_11_2: const #8444u : u16
        let s_11_2: u16 = 8444;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 16u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #1376u : u32
        let s_12_0: u32 = 1376;
        // D s_12_1: read-reg s_12_0:u32
        let s_12_1: u32 = {
            let value = state.read_register::<u32>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var gs#331318 <= s_12_1
        fn_state.gs_331318 = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #32s : i
        let s_13_0: i128 = 32;
        // S s_13_1: call Zeros(s_13_0)
        let s_13_1: Bits = Zeros(state, tracer, s_13_0);
        // S s_13_2: cast reint s_13_1 -> u32
        let s_13_2: u32 = (s_13_1.value() as u32);
        // D s_13_3: write-var gs#331318 <= s_13_2
        fn_state.gs_331318 = s_13_2;
        // N s_13_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
