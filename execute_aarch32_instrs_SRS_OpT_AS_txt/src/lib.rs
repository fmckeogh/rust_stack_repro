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
use BadMode::*;
use LR_read::*;
use ELUsingAArch32::*;
use AArch64_MonitorModeTrap::*;
use Rmode_read::*;
use CurrentSecurityState::*;
use Rmode_set::*;
use MemA_set::*;
use SPSR_read::*;
use common::*;
pub fn execute_aarch32_instrs_SRS_OpT_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    increment_name: bool,
    mode: u8,
    wback: bool,
    wordhigher: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        base: u32,
        address: u32,
        gs_323604: bool,
        ga_364309: u32,
        gs_323602: bool,
        increment_name: bool,
        mode: u8,
        wback: bool,
        wordhigher: bool,
    }
    let fn_state = FunctionState {
        increment_name,
        mode,
        wback,
        wordhigher,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b34 b1
        if s_0_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16983u : u32
        let s_1_0: u32 = 16983;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 5u16);
        // C s_1_3: const #352u : u32
        let s_1_3: u32 = 352;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b33 b2
        if s_1_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16983u : u32
        let s_2_0: u32 = 16983;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // C s_2_3: const #416u : u32
        let s_2_3: u32 = 416;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 5u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // D s_2_7: write-var gs#323602 <= s_2_6
        fn_state.gs_323602 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#323602:u8
        let s_3_0: bool = fn_state.gs_323602;
        // N s_3_1: branch s_3_0 b32 b4
        if s_3_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var mode:u8
        let s_4_0: u8 = fn_state.mode;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // C s_4_2: const #400u : u32
        let s_4_2: u32 = 400;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 5u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b31 b5
        if s_4_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var mode:u8
        let s_5_0: u8 = fn_state.mode;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_2: const #384u : u32
        let s_5_2: u32 = 384;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 5u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b23 b6
        if s_5_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var mode:u8
        let s_6_0: u8 = fn_state.mode;
        // D s_6_1: call BadMode(s_6_0)
        let s_6_1: bool = BadMode(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b22 b7
        if s_6_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #13s : i
        let s_8_0: i128 = 13;
        // D s_8_1: read-var mode:u8
        let s_8_1: u8 = fn_state.mode;
        // D s_8_2: call Rmode_read(s_8_0, s_8_1)
        let s_8_2: u32 = Rmode_read(state, tracer, s_8_0, s_8_1);
        // D s_8_3: write-var base <= s_8_2
        fn_state.base = s_8_2;
        // D s_8_4: read-var increment_name:u8
        let s_8_4: bool = fn_state.increment_name;
        // N s_8_5: branch s_8_4 b21 b9
        if s_8_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #8s : i
        let s_9_0: i128 = 8;
        // D s_9_1: read-var base:u32
        let s_9_1: u32 = fn_state.base;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 32u16);
        // C s_9_3: cast cvt s_9_0 -> bv
        let s_9_3: Bits = Bits::new(s_9_0 as u128, 128);
        // D s_9_4: sub s_9_2 s_9_3
        let s_9_4: Bits = ((s_9_2) - (s_9_3));
        // D s_9_5: cast reint s_9_4 -> u32
        let s_9_5: u32 = (s_9_4.value() as u32);
        // D s_9_6: write-var address <= s_9_5
        fn_state.address = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var wordhigher:u8
        let s_10_0: bool = fn_state.wordhigher;
        // N s_10_1: branch s_10_0 b20 b11
        if s_10_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call LR_read(s_12_0)
        let s_12_1: u32 = LR_read(state, tracer, s_12_0);
        // C s_12_2: const #4s : i
        let s_12_2: i128 = 4;
        // S s_12_3: cast zx s_12_1 -> bv
        let s_12_3: Bits = Bits::new(s_12_1 as u128, 32u16);
        // D s_12_4: read-var address:u32
        let s_12_4: u32 = fn_state.address;
        // D s_12_5: call MemA_set(s_12_4, s_12_2, s_12_3)
        let s_12_5: () = MemA_set(state, tracer, s_12_4, s_12_2, s_12_3);
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #4s : i
        let s_13_0: i128 = 4;
        // D s_13_1: read-var address:u32
        let s_13_1: u32 = fn_state.address;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 32u16);
        // C s_13_3: cast cvt s_13_0 -> bv
        let s_13_3: Bits = Bits::new(s_13_0 as u128, 128);
        // D s_13_4: add s_13_2 s_13_3
        let s_13_4: Bits = (s_13_2 + s_13_3);
        // D s_13_5: cast reint s_13_4 -> u32
        let s_13_5: u32 = (s_13_4.value() as u32);
        // C s_13_6: const #32s : i64
        let s_13_6: i64 = 32;
        // C s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // S s_13_8: call SPSR_read(s_13_7)
        let s_13_8: Bits = SPSR_read(state, tracer, s_13_7);
        // S s_13_9: cast reint s_13_8 -> u32
        let s_13_9: u32 = (s_13_8.value() as u32);
        // C s_13_10: const #4s : i
        let s_13_10: i128 = 4;
        // S s_13_11: cast zx s_13_9 -> bv
        let s_13_11: Bits = Bits::new(s_13_9 as u128, 32u16);
        // D s_13_12: call MemA_set(s_13_5, s_13_10, s_13_11)
        let s_13_12: () = MemA_set(state, tracer, s_13_5, s_13_10, s_13_11);
        // N s_13_13: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var wback:u8
        let s_14_0: bool = fn_state.wback;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var increment_name:u8
        let s_16_0: bool = fn_state.increment_name;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #8s : i
        let s_17_0: i128 = 8;
        // D s_17_1: read-var base:u32
        let s_17_1: u32 = fn_state.base;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 32u16);
        // C s_17_3: cast cvt s_17_0 -> bv
        let s_17_3: Bits = Bits::new(s_17_0 as u128, 128);
        // D s_17_4: sub s_17_2 s_17_3
        let s_17_4: Bits = ((s_17_2) - (s_17_3));
        // D s_17_5: cast reint s_17_4 -> u32
        let s_17_5: u32 = (s_17_4.value() as u32);
        // D s_17_6: write-var ga#364309 <= s_17_5
        fn_state.ga_364309 = s_17_5;
        // N s_17_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #13s : i
        let s_18_0: i128 = 13;
        // D s_18_1: read-var mode:u8
        let s_18_1: u8 = fn_state.mode;
        // D s_18_2: read-var ga#364309:u32
        let s_18_2: u32 = fn_state.ga_364309;
        // D s_18_3: call Rmode_set(s_18_0, s_18_1, s_18_2)
        let s_18_3: () = Rmode_set(state, tracer, s_18_0, s_18_1, s_18_2);
        // N s_18_4: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #8s : i
        let s_19_0: i128 = 8;
        // D s_19_1: read-var base:u32
        let s_19_1: u32 = fn_state.base;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 32u16);
        // C s_19_3: cast cvt s_19_0 -> bv
        let s_19_3: Bits = Bits::new(s_19_0 as u128, 128);
        // D s_19_4: add s_19_2 s_19_3
        let s_19_4: Bits = (s_19_2 + s_19_3);
        // D s_19_5: cast reint s_19_4 -> u32
        let s_19_5: u32 = (s_19_4.value() as u32);
        // D s_19_6: write-var ga#364309 <= s_19_5
        fn_state.ga_364309 = s_19_5;
        // N s_19_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #4s : i
        let s_20_0: i128 = 4;
        // D s_20_1: read-var address:u32
        let s_20_1: u32 = fn_state.address;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 32u16);
        // C s_20_3: cast cvt s_20_0 -> bv
        let s_20_3: Bits = Bits::new(s_20_0 as u128, 128);
        // D s_20_4: add s_20_2 s_20_3
        let s_20_4: Bits = (s_20_2 + s_20_3);
        // D s_20_5: cast reint s_20_4 -> u32
        let s_20_5: u32 = (s_20_4.value() as u32);
        // D s_20_6: write-var address <= s_20_5
        fn_state.address = s_20_5;
        // N s_20_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var base:u32
        let s_21_0: u32 = fn_state.base;
        // D s_21_1: write-var address <= s_21_0
        fn_state.address = s_21_0;
        // N s_21_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // D s_23_3: cmp-lt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) < (s_23_2));
        // D s_23_4: not s_23_3
        let s_23_4: bool = !s_23_3;
        // N s_23_5: branch s_23_4 b30 b24
        if s_23_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call CurrentSecurityState(s_24_0)
        let s_24_1: u32 = CurrentSecurityState(state, tracer, s_24_0);
        // C s_24_2: const #3u : u32
        let s_24_2: u32 = 3;
        // S s_24_3: cmp-eq s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) == (s_24_2));
        // D s_24_4: write-var gs#323604 <= s_24_3
        fn_state.gs_323604 = s_24_3;
        // N s_24_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#323604:u8
        let s_25_0: bool = fn_state.gs_323604;
        // N s_25_1: branch s_25_0 b29 b26
        if s_25_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #424u : u32
        let s_26_0: u32 = 424;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call ELUsingAArch32(s_26_1)
        let s_26_2: bool = ELUsingAArch32(state, tracer, s_26_1);
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // N s_26_4: branch s_26_3 b28 b27
        if s_26_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call AArch64_MonitorModeTrap(s_28_0)
        let s_28_1: () = AArch64_MonitorModeTrap(state, tracer, s_28_0);
        // N s_28_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#323604 <= s_30_0
        fn_state.gs_323604 = s_30_0;
        // N s_30_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#323602 <= s_33_0
        fn_state.gs_323602 = s_33_0;
        // N s_33_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
}
