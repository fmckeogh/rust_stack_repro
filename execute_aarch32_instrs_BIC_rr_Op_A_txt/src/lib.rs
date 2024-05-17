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
use Shift_C::*;
use R_read::*;
use R_set::*;
use IsZeroBit::*;
use common::*;
pub fn execute_aarch32_instrs_BIC_rr_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    s: i64,
    setflags: bool,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_878445: ProductTypef506aa96a892fe52,
        carry: bool,
        result: u32,
        d: i64,
        m: i64,
        n: i64,
        s: i64,
        setflags: bool,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        s,
        setflags,
        shift_t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var s:i64
        let s_0_0: i64 = fn_state.s;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 8u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call R_read(s_0_15)
        let s_0_16: u32 = R_read(state, tracer, s_0_15);
        // C s_0_17: const #16971u : u32
        let s_0_17: u32 = 16971;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: bool = {
            let value = state.read_register::<bool>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_16 -> bv
        let s_0_19: Bits = Bits::new(s_0_16 as u128, 32u16);
        // D s_0_20: cast zx s_0_13 -> i
        let s_0_20: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_21: read-var shift_t:u32
        let s_0_21: u32 = fn_state.shift_t;
        // D s_0_22: call Shift_C(s_0_19, s_0_21, s_0_20, s_0_18)
        let s_0_22: ProductTypef506aa96a892fe52 = Shift_C(
            state,
            tracer,
            s_0_19,
            s_0_21,
            s_0_20,
            s_0_18,
        );
        // D s_0_23: write-var gs#878445 <= s_0_22
        fn_state.gs_878445 = s_0_22;
        // D s_0_24: read-var gs#878445.0:struct
        let s_0_24: Bits = fn_state.gs_878445._0;
        // D s_0_25: cast reint s_0_24 -> u32
        let s_0_25: u32 = (s_0_24.value() as u32);
        // D s_0_26: read-var gs#878445.1:struct
        let s_0_26: bool = fn_state.gs_878445._1;
        // D s_0_27: write-var carry <= s_0_26
        fn_state.carry = s_0_26;
        // D s_0_28: read-var n:i64
        let s_0_28: i64 = fn_state.n;
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_30: call R_read(s_0_29)
        let s_0_30: u32 = R_read(state, tracer, s_0_29);
        // D s_0_31: cast zx s_0_25 -> bv
        let s_0_31: Bits = Bits::new(s_0_25 as u128, 32u16);
        // D s_0_32: not s_0_31
        let s_0_32: Bits = !s_0_31;
        // D s_0_33: cast reint s_0_32 -> u32
        let s_0_33: u32 = (s_0_32.value() as u32);
        // D s_0_34: cast zx s_0_30 -> bv
        let s_0_34: Bits = Bits::new(s_0_30 as u128, 32u16);
        // D s_0_35: cast zx s_0_33 -> bv
        let s_0_35: Bits = Bits::new(s_0_33 as u128, 32u16);
        // D s_0_36: and s_0_34 s_0_35
        let s_0_36: Bits = ((s_0_34) & (s_0_35));
        // D s_0_37: cast reint s_0_36 -> u32
        let s_0_37: u32 = (s_0_36.value() as u32);
        // D s_0_38: write-var result <= s_0_37
        fn_state.result = s_0_37;
        // D s_0_39: read-var d:i64
        let s_0_39: i64 = fn_state.d;
        // D s_0_40: cast zx s_0_39 -> i
        let s_0_40: i128 = (i128::try_from(s_0_39).unwrap());
        // D s_0_41: read-var result:u32
        let s_0_41: u32 = fn_state.result;
        // D s_0_42: call R_set(s_0_40, s_0_41)
        let s_0_42: () = R_set(state, tracer, s_0_40, s_0_41);
        // D s_0_43: read-var setflags:u8
        let s_0_43: bool = fn_state.setflags;
        // N s_0_44: branch s_0_43 b2 b1
        if s_0_43 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var result:u32
        let s_2_1: u32 = fn_state.result;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // C s_2_18: const #16984u : u32
        let s_2_18: u32 = 16984;
        // N s_2_19: write-reg s_2_18 <= s_2_17
        let s_2_19: () = {
            state.write_register::<bool>(s_2_18 as isize, s_2_17);
            tracer.write_register(s_2_18 as isize, s_2_17);
        };
        // D s_2_20: read-var result:u32
        let s_2_20: u32 = fn_state.result;
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 32u16);
        // D s_2_22: call IsZeroBit(s_2_21)
        let s_2_22: bool = IsZeroBit(state, tracer, s_2_21);
        // C s_2_23: const #16997u : u32
        let s_2_23: u32 = 16997;
        // N s_2_24: write-reg s_2_23 <= s_2_22
        let s_2_24: () = {
            state.write_register::<bool>(s_2_23 as isize, s_2_22);
            tracer.write_register(s_2_23 as isize, s_2_22);
        };
        // D s_2_25: read-var carry:u8
        let s_2_25: bool = fn_state.carry;
        // C s_2_26: const #16971u : u32
        let s_2_26: u32 = 16971;
        // N s_2_27: write-reg s_2_26 <= s_2_25
        let s_2_27: () = {
            state.write_register::<bool>(s_2_26 as isize, s_2_25);
            tracer.write_register(s_2_26 as isize, s_2_25);
        };
        // N s_2_28: return
        return;
    }
}
