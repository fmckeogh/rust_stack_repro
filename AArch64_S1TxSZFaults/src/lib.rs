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
use AArch64_MaxTxSZ::*;
use u__IMPDEF_boolean::*;
use AArch64_S1MinTxSZ::*;
use Have52BitVAExt::*;
use common::*;
pub fn AArch64_S1TxSZFaults<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        maxtxsz: i128,
        return_value: bool,
        gs_13552: bool,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
    }
    let fn_state = FunctionState {
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
        // D s_0_0: read-var walkparams.3:struct
        let s_0_0: bool = fn_state.walkparams._3;
        // D s_0_1: read-var walkparams.7:struct
        let s_0_1: bool = fn_state.walkparams._7;
        // D s_0_2: read-var walkparams.36:struct
        let s_0_2: u32 = fn_state.walkparams._36;
        // D s_0_3: read-var regime:u32
        let s_0_3: u32 = fn_state.regime;
        // D s_0_4: call AArch64_S1MinTxSZ(s_0_3, s_0_0, s_0_1, s_0_2)
        let s_0_4: i128 = AArch64_S1MinTxSZ(state, tracer, s_0_3, s_0_0, s_0_1, s_0_2);
        // D s_0_5: read-var walkparams.36:struct
        let s_0_5: u32 = fn_state.walkparams._36;
        // D s_0_6: call AArch64_MaxTxSZ(s_0_5)
        let s_0_6: i128 = AArch64_MaxTxSZ(state, tracer, s_0_5);
        // D s_0_7: write-var maxtxsz <= s_0_6
        fn_state.maxtxsz = s_0_6;
        // D s_0_8: read-var walkparams.37:struct
        let s_0_8: u8 = fn_state.walkparams._37;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 6u16);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: cmp-lt s_0_12 s_0_4
        let s_0_13: bool = ((s_0_12) < (s_0_4));
        // N s_0_14: branch s_0_13 b5 b1
        if s_0_13 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var walkparams.37:struct
        let s_1_0: u8 = fn_state.walkparams._37;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 6u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var maxtxsz:i
        let s_1_5: i128 = fn_state.maxtxsz;
        // D s_1_6: cmp-gt s_1_4 s_1_5
        let s_1_6: bool = ((s_1_4) > (s_1_5));
        // N s_1_7: branch s_1_6 b4 b2
        if s_1_6 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var return_value <= s_2_0
        fn_state.return_value = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var return_value:u8
        let s_3_0: bool = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #"Fault on TxSZ value above maximum" : str
        let s_4_0: &'static str = "Fault on TxSZ value above maximum";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // D s_4_2: write-var return_value <= s_4_1
        fn_state.return_value = s_4_1;
        // N s_4_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call Have52BitVAExt(s_5_0)
        let s_5_1: bool = Have52BitVAExt(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b8 b6
        if s_5_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #"Fault on TxSZ value below minimum" : str
        let s_6_0: &'static str = "Fault on TxSZ value below minimum";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // D s_6_2: write-var gs#13552 <= s_6_1
        fn_state.gs_13552 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#13552:u8
        let s_7_0: bool = fn_state.gs_13552;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#13552 <= s_8_0
        fn_state.gs_13552 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
}
