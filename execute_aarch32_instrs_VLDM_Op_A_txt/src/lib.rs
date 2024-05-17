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
use MemA_read::*;
use CheckVFPEnabled::*;
use R_read::*;
use R_set::*;
use BigEndian::*;
use D_set::*;
use S_set::*;
use common::*;
pub fn execute_aarch32_instrs_VLDM_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d: i64,
    imm32: u32,
    n: i64,
    regs: i128,
    single_regs: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        ga_355127: u64,
        gs_311751: i64,
        ga_355126: i128,
        gs_897820: Bits,
        address: u32,
        word2: u32,
        gs_897816: Bits,
        ga_355122: i128,
        word1: u32,
        ga_355130: u32,
        gs_897832: Bits,
        add: bool,
        d: i64,
        imm32: u32,
        n: i64,
        regs: i128,
        single_regs: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        add,
        d,
        imm32,
        n,
        regs,
        single_regs,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var add:u8
        let s_1_0: bool = fn_state.add;
        // N s_1_1: branch s_1_0 b21 b2
        if s_1_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 32u16);
        // D s_2_4: read-var imm32:u32
        let s_2_4: u32 = fn_state.imm32;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 32u16);
        // D s_2_6: sub s_2_3 s_2_5
        let s_2_6: Bits = ((s_2_3) - (s_2_5));
        // D s_2_7: cast reint s_2_6 -> u32
        let s_2_7: u32 = (s_2_6.value() as u32);
        // D s_2_8: write-var address <= s_2_7
        fn_state.address = s_2_7;
        // N s_2_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var regs:i
        let s_3_2: i128 = fn_state.regs;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#311751 <= s_3_4
        fn_state.gs_311751 = s_3_4;
        // D s_3_6: write-var r <= s_3_0
        fn_state.r = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#311751:i64
        let s_4_1: i64 = fn_state.gs_311751;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var single_regs:u8
        let s_5_0: bool = fn_state.single_regs;
        // N s_5_1: branch s_5_0 b13 b6
        if s_5_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4s : i64
        let s_6_0: i64 = 4;
        // D s_6_1: read-var address:u32
        let s_6_1: u32 = fn_state.address;
        // D s_6_2: call MemA_read(s_6_1, s_6_0)
        let s_6_2: Bits = MemA_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: write-var gs#897816 <= s_6_2
        fn_state.gs_897816 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#897816:bv
        let s_7_0: Bits = fn_state.gs_897816;
        // D s_7_1: cast reint s_7_0 -> u32
        let s_7_1: u32 = (s_7_0.value() as u32);
        // D s_7_2: write-var word1 <= s_7_1
        fn_state.word1 = s_7_1;
        // C s_7_3: const #4s : i
        let s_7_3: i128 = 4;
        // D s_7_4: read-var address:u32
        let s_7_4: u32 = fn_state.address;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 32u16);
        // C s_7_6: cast cvt s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 128);
        // D s_7_7: add s_7_5 s_7_6
        let s_7_7: Bits = (s_7_5 + s_7_6);
        // D s_7_8: cast reint s_7_7 -> u32
        let s_7_8: u32 = (s_7_7.value() as u32);
        // C s_7_9: const #4s : i64
        let s_7_9: i64 = 4;
        // D s_7_10: call MemA_read(s_7_8, s_7_9)
        let s_7_10: Bits = MemA_read(state, tracer, s_7_8, s_7_9);
        // D s_7_11: write-var gs#897820 <= s_7_10
        fn_state.gs_897820 = s_7_10;
        // N s_7_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#897820:bv
        let s_8_0: Bits = fn_state.gs_897820;
        // D s_8_1: cast reint s_8_0 -> u32
        let s_8_1: u32 = (s_8_0.value() as u32);
        // D s_8_2: write-var word2 <= s_8_1
        fn_state.word2 = s_8_1;
        // C s_8_3: const #8s : i
        let s_8_3: i128 = 8;
        // D s_8_4: read-var address:u32
        let s_8_4: u32 = fn_state.address;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 32u16);
        // C s_8_6: cast cvt s_8_3 -> bv
        let s_8_6: Bits = Bits::new(s_8_3 as u128, 128);
        // D s_8_7: add s_8_5 s_8_6
        let s_8_7: Bits = (s_8_5 + s_8_6);
        // D s_8_8: cast reint s_8_7 -> u32
        let s_8_8: u32 = (s_8_7.value() as u32);
        // D s_8_9: write-var address <= s_8_8
        fn_state.address = s_8_8;
        // D s_8_10: read-var d:i64
        let s_8_10: i64 = fn_state.d;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var r:i64
        let s_8_12: i64 = fn_state.r;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: add s_8_11 s_8_13
        let s_8_14: i128 = (s_8_11 + s_8_13);
        // D s_8_15: write-var ga#355126 <= s_8_14
        fn_state.ga_355126 = s_8_14;
        // C s_8_16: const #2u : u32
        let s_8_16: u32 = 2;
        // S s_8_17: call BigEndian(s_8_16)
        let s_8_17: bool = BigEndian(state, tracer, s_8_16);
        // N s_8_18: branch s_8_17 b12 b9
        if s_8_17 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var word2:u32
        let s_9_0: u32 = fn_state.word2;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 32u16);
        // D s_9_2: read-var word1:u32
        let s_9_2: u32 = fn_state.word1;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 32u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u64
        let s_9_12: u64 = (s_9_11.value() as u64);
        // D s_9_13: write-var ga#355127 <= s_9_12
        fn_state.ga_355127 = s_9_12;
        // N s_9_14: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#355126:i
        let s_10_0: i128 = fn_state.ga_355126;
        // D s_10_1: read-var ga#355127:u64
        let s_10_1: u64 = fn_state.ga_355127;
        // D s_10_2: call D_set(s_10_0, s_10_1)
        let s_10_2: () = D_set(state, tracer, s_10_0, s_10_1);
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var r:i64
        let s_11_0: i64 = fn_state.r;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var r <= s_11_2
        fn_state.r = s_11_2;
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var word1:u32
        let s_12_0: u32 = fn_state.word1;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 32u16);
        // D s_12_2: read-var word2:u32
        let s_12_2: u32 = fn_state.word2;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 32u16);
        // D s_12_4: cast reint s_12_1 -> u128
        let s_12_4: u128 = (s_12_1.value() as u128);
        // D s_12_5: size-of s_12_1
        let s_12_5: u16 = s_12_1.length();
        // D s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // D s_12_8: lsl s_12_4 s_12_7
        let s_12_8: u128 = s_12_4 << s_12_7;
        // D s_12_9: or s_12_8 s_12_6
        let s_12_9: u128 = ((s_12_8) | (s_12_6));
        // D s_12_10: add s_12_5 s_12_7
        let s_12_10: u16 = (s_12_5 + s_12_7);
        // D s_12_11: create-bits s_12_9 s_12_10
        let s_12_11: Bits = Bits::new(s_12_9, s_12_10);
        // D s_12_12: cast reint s_12_11 -> u64
        let s_12_12: u64 = (s_12_11.value() as u64);
        // D s_12_13: write-var ga#355127 <= s_12_12
        fn_state.ga_355127 = s_12_12;
        // N s_12_14: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var r:i64
        let s_13_2: i64 = fn_state.r;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: add s_13_1 s_13_3
        let s_13_4: i128 = (s_13_1 + s_13_3);
        // D s_13_5: write-var ga#355122 <= s_13_4
        fn_state.ga_355122 = s_13_4;
        // C s_13_6: const #4s : i64
        let s_13_6: i64 = 4;
        // D s_13_7: read-var address:u32
        let s_13_7: u32 = fn_state.address;
        // D s_13_8: call MemA_read(s_13_7, s_13_6)
        let s_13_8: Bits = MemA_read(state, tracer, s_13_7, s_13_6);
        // D s_13_9: write-var gs#897832 <= s_13_8
        fn_state.gs_897832 = s_13_8;
        // N s_13_10: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#897832:bv
        let s_14_0: Bits = fn_state.gs_897832;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: read-var ga#355122:i
        let s_14_2: i128 = fn_state.ga_355122;
        // D s_14_3: call S_set(s_14_2, s_14_1)
        let s_14_3: () = S_set(state, tracer, s_14_2, s_14_1);
        // C s_14_4: const #4s : i
        let s_14_4: i128 = 4;
        // D s_14_5: read-var address:u32
        let s_14_5: u32 = fn_state.address;
        // D s_14_6: cast zx s_14_5 -> bv
        let s_14_6: Bits = Bits::new(s_14_5 as u128, 32u16);
        // C s_14_7: cast cvt s_14_4 -> bv
        let s_14_7: Bits = Bits::new(s_14_4 as u128, 128);
        // D s_14_8: add s_14_6 s_14_7
        let s_14_8: Bits = (s_14_6 + s_14_7);
        // D s_14_9: cast reint s_14_8 -> u32
        let s_14_9: u32 = (s_14_8.value() as u32);
        // D s_14_10: write-var address <= s_14_9
        fn_state.address = s_14_9;
        // N s_14_11: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var wback:u8
        let s_15_0: bool = fn_state.wback;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var add:u8
        let s_17_0: bool = fn_state.add;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call R_read(s_18_1)
        let s_18_2: u32 = R_read(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 32u16);
        // D s_18_4: read-var imm32:u32
        let s_18_4: u32 = fn_state.imm32;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 32u16);
        // D s_18_6: sub s_18_3 s_18_5
        let s_18_6: Bits = ((s_18_3) - (s_18_5));
        // D s_18_7: cast reint s_18_6 -> u32
        let s_18_7: u32 = (s_18_6.value() as u32);
        // D s_18_8: write-var ga#355130 <= s_18_7
        fn_state.ga_355130 = s_18_7;
        // N s_18_9: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var ga#355130:u32
        let s_19_2: u32 = fn_state.ga_355130;
        // D s_19_3: call R_set(s_19_1, s_19_2)
        let s_19_3: () = R_set(state, tracer, s_19_1, s_19_2);
        // N s_19_4: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var n:i64
        let s_20_0: i64 = fn_state.n;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call R_read(s_20_1)
        let s_20_2: u32 = R_read(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 32u16);
        // D s_20_4: read-var imm32:u32
        let s_20_4: u32 = fn_state.imm32;
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 32u16);
        // D s_20_6: add s_20_3 s_20_5
        let s_20_6: Bits = (s_20_3 + s_20_5);
        // D s_20_7: cast reint s_20_6 -> u32
        let s_20_7: u32 = (s_20_6.value() as u32);
        // D s_20_8: write-var ga#355130 <= s_20_7
        fn_state.ga_355130 = s_20_7;
        // N s_20_9: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call R_read(s_21_1)
        let s_21_2: u32 = R_read(state, tracer, s_21_1);
        // D s_21_3: write-var address <= s_21_2
        fn_state.address = s_21_2;
        // N s_21_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
