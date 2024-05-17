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
use AArch64_IASize::*;
use is_zero_subrange::*;
use AArch64_AddrTop::*;
use AArch64_GetVARange::*;
use is_ones_subrange::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_VAIsOutOfRange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va_in: u64,
    acctype: u32,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        va: u64,
        iasize: i128,
        ga_15186: u8,
        addrtop: i128,
        va_in: u64,
        acctype: u32,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
    }
    let fn_state = FunctionState {
        va_in,
        acctype,
        regime,
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var va_in:u64
        let s_0_0: u64 = fn_state.va_in;
        // D s_0_1: write-var va <= s_0_0
        fn_state.va = s_0_0;
        // D s_0_2: read-var walkparams.35:struct
        let s_0_2: bool = fn_state.walkparams._35;
        // D s_0_3: read-var walkparams.34:struct
        let s_0_3: bool = fn_state.walkparams._34;
        // D s_0_4: read-var acctype:u32
        let s_0_4: u32 = fn_state.acctype;
        // D s_0_5: call AArch64_AddrTop(s_0_2, s_0_4, s_0_3)
        let s_0_5: i128 = AArch64_AddrTop(state, tracer, s_0_2, s_0_4, s_0_3);
        // D s_0_6: write-var addrtop <= s_0_5
        fn_state.addrtop = s_0_5;
        // D s_0_7: read-var walkparams.19:struct
        let s_0_7: bool = fn_state.walkparams._19;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // C s_0_9: const #1u : u8
        let s_0_9: bool = true;
        // C s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // D s_0_11: cmp-eq s_0_8 s_0_10
        let s_0_11: bool = ((s_0_8) == (s_0_10));
        // N s_0_12: branch s_0_11 b10 b1
        if s_0_11 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var walkparams.37:struct
        let s_2_0: u8 = fn_state.walkparams._37;
        // D s_2_1: call AArch64_IASize(s_2_0)
        let s_2_1: i128 = AArch64_IASize(state, tracer, s_2_0);
        // D s_2_2: write-var iasize <= s_2_1
        fn_state.iasize = s_2_1;
        // D s_2_3: read-var addrtop:i
        let s_2_3: i128 = fn_state.addrtop;
        // D s_2_4: read-var iasize:i
        let s_2_4: i128 = fn_state.iasize;
        // D s_2_5: cmp-lt s_2_3 s_2_4
        let s_2_5: bool = ((s_2_3) < (s_2_4));
        // N s_2_6: branch s_2_5 b9 b3
        if s_2_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var regime:u32
        let s_3_0: u32 = fn_state.regime;
        // D s_3_1: call HasUnprivileged(s_3_0)
        let s_3_1: bool = HasUnprivileged(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b6 b4
        if s_3_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var va:u64
        let s_4_0: u64 = fn_state.va;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var addrtop:i
        let s_4_2: i128 = fn_state.addrtop;
        // D s_4_3: read-var iasize:i
        let s_4_3: i128 = fn_state.iasize;
        // D s_4_4: call is_zero_subrange(s_4_1, s_4_2, s_4_3)
        let s_4_4: bool = is_zero_subrange(state, tracer, s_4_1, s_4_2, s_4_3);
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // D s_4_6: write-var return_value <= s_4_5
        fn_state.return_value = s_4_5;
        // N s_4_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var va:u64
        let s_6_0: u64 = fn_state.va;
        // D s_6_1: call AArch64_GetVARange(s_6_0)
        let s_6_1: u32 = AArch64_GetVARange(state, tracer, s_6_0);
        // C s_6_2: const #0u : u32
        let s_6_2: u32 = 0;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var va:u64
        let s_7_0: u64 = fn_state.va;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var addrtop:i
        let s_7_2: i128 = fn_state.addrtop;
        // D s_7_3: read-var iasize:i
        let s_7_3: i128 = fn_state.iasize;
        // D s_7_4: call is_ones_subrange(s_7_1, s_7_2, s_7_3)
        let s_7_4: bool = is_ones_subrange(state, tracer, s_7_1, s_7_2, s_7_3);
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // D s_7_6: write-var return_value <= s_7_5
        fn_state.return_value = s_7_5;
        // N s_7_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var va:u64
        let s_8_0: u64 = fn_state.va;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_2: read-var addrtop:i
        let s_8_2: i128 = fn_state.addrtop;
        // D s_8_3: read-var iasize:i
        let s_8_3: i128 = fn_state.iasize;
        // D s_8_4: call is_zero_subrange(s_8_1, s_8_2, s_8_3)
        let s_8_4: bool = is_zero_subrange(state, tracer, s_8_1, s_8_2, s_8_3);
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // D s_8_6: write-var return_value <= s_8_5
        fn_state.return_value = s_8_5;
        // N s_8_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var va:u64
        let s_10_0: u64 = fn_state.va;
        // D s_10_1: call AArch64_GetVARange(s_10_0)
        let s_10_1: u32 = AArch64_GetVARange(state, tracer, s_10_0);
        // C s_10_2: const #1u : u32
        let s_10_2: u32 = 1;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // N s_10_4: branch s_10_3 b13 b11
        if s_10_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: u8 = 0;
        // D s_11_1: write-var ga#15186 <= s_11_0
        fn_state.ga_15186 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #56s : i
        let s_12_0: i128 = 56;
        // D s_12_1: read-var va:u64
        let s_12_1: u64 = fn_state.va;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 64u16);
        // D s_12_3: read-var ga#15186:u8
        let s_12_3: u8 = fn_state.ga_15186;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 4u16);
        // C s_12_5: const #3s : i
        let s_12_5: i128 = 3;
        // C s_12_6: const #1u : u64
        let s_12_6: u64 = 1;
        // C s_12_7: cast zx s_12_6 -> bv
        let s_12_7: Bits = Bits::new(s_12_6 as u128, 64u16);
        // C s_12_8: lsl s_12_7 s_12_5
        let s_12_8: Bits = s_12_7 << s_12_5;
        // C s_12_9: sub s_12_8 s_12_7
        let s_12_9: Bits = ((s_12_8) - (s_12_7));
        // D s_12_10: and s_12_4 s_12_9
        let s_12_10: Bits = ((s_12_4) & (s_12_9));
        // D s_12_11: lsl s_12_10 s_12_0
        let s_12_11: Bits = s_12_10 << s_12_0;
        // C s_12_12: lsl s_12_9 s_12_0
        let s_12_12: Bits = s_12_9 << s_12_0;
        // C s_12_13: cmpl s_12_12
        let s_12_13: Bits = !s_12_12;
        // D s_12_14: and s_12_2 s_12_13
        let s_12_14: Bits = ((s_12_2) & (s_12_13));
        // D s_12_15: or s_12_14 s_12_11
        let s_12_15: Bits = ((s_12_14) | (s_12_11));
        // D s_12_16: cast reint s_12_15 -> u64
        let s_12_16: u64 = (s_12_15.value() as u64);
        // D s_12_17: write-var va <= s_12_16
        fn_state.va = s_12_16;
        // N s_12_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #15u : u8
        let s_13_0: u8 = 15;
        // D s_13_1: write-var ga#15186 <= s_13_0
        fn_state.ga_15186 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
}
